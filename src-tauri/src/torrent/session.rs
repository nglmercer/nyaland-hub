use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use librqbit::api::TorrentIdOrHash;
use librqbit::{
    AddTorrent, AddTorrentOptions, DhtSessionConfig, PeerConnectionOptions, Session,
    SessionOptions, SessionPersistenceConfig,
};
use tokio::sync::RwLock;

use super::handle::{TorrentHandle, TorrentSessionInner};
use super::state::DownloadState;
use super::utils::{extract_hash_from_magnet, extract_trackers_from_magnet, hash_fallback, truncate_magnet};
use crate::types::{resolve_save_path, AppSettings};

#[derive(Clone)]
pub struct TorrentSession {
    pub inner: Arc<RwLock<TorrentSessionInner>>,
    session: Arc<Session>,
}

impl TorrentSession {
    pub async fn new(settings: AppSettings) -> Self {
        let download_dir = if cfg!(target_os = "android") {
            PathBuf::from("/data/user/0/com.nyaland.desktop/files/NyaHub")
        } else {
            resolve_save_path(&settings.save_path)
        };
        std::fs::create_dir_all(&download_dir).ok();

        let session_opts = SessionOptions {
            persistence: if cfg!(target_os = "android") {
                None
            } else {
                Some(SessionPersistenceConfig::Json {
                    folder: Some(download_dir.join(".rqbit-session")),
                })
            },
            dht: if cfg!(target_os = "android") {
                None
            } else {
                let mut dht_cfg = DhtSessionConfig::default();
                dht_cfg.bootstrap_addrs = Some(vec![
                    "router.bittorrent.com:6881".to_string(),
                    "dht.transmissionbt.com:6881".to_string(),
                    "router.utorrent.com:6881".to_string(),
                    "dht.libtorrent.org:25401".to_string(),
                    "dht.aelitis.com:6881".to_string(),
                ]);
                Some(dht_cfg)
            },
            ..Default::default()
        };

        let session = match Session::new_with_opts(download_dir.clone(), session_opts).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[nyaland] Failed to create torrent session: {e}, using fallback");
                Session::new(download_dir.clone())
                    .await
                    .expect("Failed to create fallback session")
            }
        };

        let inner = Arc::new(RwLock::new(TorrentSessionInner {
            downloads: std::collections::HashMap::new(),
            settings,
        }));

        Self { inner, session }
    }

    pub fn new_fallback(settings: AppSettings) -> Self {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(_) => tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("failed to create current-thread fallback runtime"),
        };
        let download_dir = if cfg!(target_os = "android") {
            PathBuf::from("/data/user/0/com.nyaland.desktop/files/NyaHub")
        } else {
            resolve_save_path(&settings.save_path)
        };
        std::fs::create_dir_all(&download_dir).ok();

        let session = rt.block_on(async {
            Session::new_with_opts(
                download_dir.clone(),
                SessionOptions {
                    persistence: if cfg!(target_os = "android") {
                        None
                    } else {
                        Some(SessionPersistenceConfig::Json {
                            folder: Some(download_dir.join(".rqbit-session")),
                        })
                    },
                    dht: None,
                    ..Default::default()
                },
            )
            .await
            .expect("Failed to create fallback session")
        });

        let inner = Arc::new(RwLock::new(TorrentSessionInner {
            downloads: std::collections::HashMap::new(),
            settings,
        }));

        Self { inner, session }
    }

    pub async fn add_download(&self, magnet: &str, save_path: &str) -> Result<String, String> {
        let hash = extract_hash_from_magnet(magnet)
            .unwrap_or_else(|| format!("{:x}", hash_fallback(magnet.as_bytes())));

        let resolved_path = resolve_save_path(save_path);
        std::fs::create_dir_all(&resolved_path).ok();
        let save_path_str = resolved_path.to_string_lossy().into_owned();

        eprintln!("[nyaland] add_download: hash={hash}");

        {
            let mut inner = self.inner.write().await;
            if let Some(existing) = inner.downloads.get_mut(&hash) {
                if matches!(existing.state, DownloadState::Error) {
                    existing.state = DownloadState::Connecting;
                    existing.torrent_id = None;
                    existing.error = None;
                    existing.progress = 0.0;
                    existing.download_rate = 0;
                    existing.upload_rate = 0;
                    existing.total_size = 0;
                    existing.downloaded = 0;
                    existing.num_peers = 0;
                    existing.added_date = chrono::Utc::now().to_rfc3339();
                } else {
                    return Ok(hash);
                }
            } else {
                let handle = TorrentHandle {
                    name: truncate_magnet(magnet),
                    torrent_id: None,
                    progress: 0.0,
                    download_rate: 0,
                    upload_rate: 0,
                    total_size: 0,
                    downloaded: 0,
                    num_peers: 0,
                    state: DownloadState::Connecting,
                    save_path: save_path_str.clone(),
                    added_date: chrono::Utc::now().to_rfc3339(),
                    magnet: magnet.to_string(),
                    error: None,
                };
                inner.downloads.insert(hash.clone(), handle);
            }
        }

        let session = self.session.clone();
        let inner = self.inner.clone();
        let magnet_owned = magnet.to_string();
        let hash_clone = hash.clone();

        tokio::spawn(async move {
            let mut trackers = extract_trackers_from_magnet(&magnet_owned);
            trackers.sort_unstable();
            trackers.dedup();

            let opts = AddTorrentOptions {
                overwrite: true,
                output_folder: Some(save_path_str.clone()),
                trackers: Some(trackers),
                peer_opts: Some(PeerConnectionOptions {
                    connect_timeout: Some(std::time::Duration::from_secs(10)),
                    read_write_timeout: Some(std::time::Duration::from_secs(15)),
                    ..Default::default()
                }),
                ..Default::default()
            };

            eprintln!("[nyaland] add_torrent: resolving magnet (no timeout)...");

            match session
                .add_torrent(AddTorrent::Url(Cow::Owned(magnet_owned)), Some(opts))
                .await
            {
                Ok(resp) => {
                    use librqbit::AddTorrentResponse;
                    match resp {
                        AddTorrentResponse::Added(id, handle) => {
                            eprintln!("[nyaland] torrent added: id={id}");
                            let mut inner = inner.write().await;
                            if let Some(h) = inner.downloads.get_mut(&hash_clone) {
                                h.torrent_id = Some(id);
                                h.name = handle.name().unwrap_or_else(|| h.name.clone());
                                let stats = handle.stats();
                                h.total_size = stats.total_bytes;
                                h.state = DownloadState::Downloading;
                            }
                        }
                        AddTorrentResponse::AlreadyManaged(id, handle) => {
                            eprintln!("[nyaland] torrent already managed: id={id}");
                            let mut inner = inner.write().await;
                            if let Some(h) = inner.downloads.get_mut(&hash_clone) {
                                h.torrent_id = Some(id);
                                h.name = handle.name().unwrap_or_else(|| h.name.clone());
                                let stats = handle.stats();
                                h.total_size = stats.total_bytes;
                                h.state = DownloadState::Downloading;
                            }
                        }
                        AddTorrentResponse::ListOnly(_) => {
                            eprintln!("[nyaland] torrent list-only response");
                            let mut inner = inner.write().await;
                            if let Some(h) = inner.downloads.get_mut(&hash_clone) {
                                h.state = DownloadState::Error;
                                h.error = Some("List-only response".to_string());
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[nyaland] add_torrent FAILED: {e}");
                    let mut inner = inner.write().await;
                    if let Some(h) = inner.downloads.get_mut(&hash_clone) {
                        h.state = DownloadState::Error;
                        h.error = Some(e.to_string());
                    }
                }
            }
        });

        Ok(hash)
    }

    pub async fn get_downloads(&self) -> Vec<crate::types::DownloadStatus> {
        let mut inner = self.inner.write().await;

        for (hash, handle) in inner.downloads.iter_mut() {
            if let Some(id) = handle.torrent_id {
                let tid = TorrentIdOrHash::Id(id);
                match self.session.get(tid) {
                    Some(mgr) => {
                        let stats = mgr.stats();
                        handle.name = mgr.name().unwrap_or_else(|| handle.name.clone());
                        handle.total_size = stats.total_bytes;
                        handle.downloaded = stats.progress_bytes;
                        handle.progress = if stats.total_bytes > 0 {
                            stats.progress_bytes as f32 / stats.total_bytes as f32
                        } else {
                            0.0
                        };

                        if let Some(ref live) = stats.live {
                            handle.download_rate = live.download_speed.as_bytes();
                            handle.upload_rate = live.upload_speed.as_bytes();
                        } else {
                            handle.download_rate = 0;
                            handle.upload_rate = 0;
                        }

                        handle.state = if stats.finished {
                            DownloadState::Finished
                        } else {
                            match stats.state {
                                librqbit::TorrentStatsState::Paused => DownloadState::Paused,
                                librqbit::TorrentStatsState::Live => {
                                    if stats.live.is_some() {
                                        DownloadState::Downloading
                                    } else {
                                        DownloadState::Connecting
                                    }
                                }
                                librqbit::TorrentStatsState::Initializing => {
                                    DownloadState::Connecting
                                }
                                librqbit::TorrentStatsState::Error => DownloadState::Error,
                            }
                        };
                    }
                    None => {
                        eprintln!(
                            "[nyaland] get_downloads: hash={hash} torrent_id={id} not found in session"
                        );
                    }
                }
            }
        }

        inner
            .downloads
            .iter()
            .map(|(hash, h)| crate::types::DownloadStatus {
                hash: hash.clone(),
                name: h.name.clone(),
                progress: h.progress,
                download_rate: h.download_rate,
                upload_rate: h.upload_rate,
                total_size: h.total_size,
                downloaded: h.downloaded,
                num_peers: h.num_peers,
                state: h.state.clone(),
                save_path: h.save_path.clone(),
                added_date: Some(h.added_date.clone()),
            })
            .collect()
    }

    pub async fn pause_download(&self, hash: &str) -> Result<bool, String> {
        let mut inner = self.inner.write().await;
        if let Some(handle) = inner.downloads.get_mut(hash) {
            if let Some(id) = handle.torrent_id {
                let tid = TorrentIdOrHash::Id(id);
                let mgr = self
                    .session
                    .get(tid)
                    .ok_or("torrent not found in session")?;
                self.session
                    .pause(&mgr)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            handle.state = DownloadState::Paused;
            handle.download_rate = 0;
            return Ok(true);
        }
        Err("Download not found".to_string())
    }

    pub async fn resume_download(&self, hash: &str) -> Result<bool, String> {
        let mut inner = self.inner.write().await;
        if let Some(handle) = inner.downloads.get_mut(hash) {
            if let Some(id) = handle.torrent_id {
                let tid = TorrentIdOrHash::Id(id);
                let mgr = self
                    .session
                    .get(tid)
                    .ok_or("torrent not found in session")?;
                self.session
                    .unpause(&mgr)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            handle.state = DownloadState::Downloading;
            return Ok(true);
        }
        Err("Download not found".to_string())
    }

    pub async fn remove_download(&self, hash: &str, delete_files: bool) -> Result<bool, String> {
        let mut inner = self.inner.write().await;
        if let Some(handle) = inner.downloads.remove(hash) {
            if let Some(id) = handle.torrent_id {
                let tid = TorrentIdOrHash::Id(id);
                self.session
                    .delete(tid, delete_files)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            return Ok(true);
        }
        Err("Download not found".to_string())
    }
}
