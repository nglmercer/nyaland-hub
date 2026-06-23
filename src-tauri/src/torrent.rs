use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use librqbit::api::TorrentIdOrHash;
use librqbit::{
    AddTorrent, AddTorrentOptions, Api, DhtSessionConfig, Session, SessionOptions,
    SessionPersistenceConfig,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::types::{resolve_save_path, AppSettings};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DownloadState {
    Queued,
    Connecting,
    Downloading,
    Paused,
    Finished,
    Error,
    Moving,
}

pub struct TorrentHandle {
    pub name: String,
    pub torrent_id: Option<usize>,
    pub progress: f32,
    pub download_rate: u64,
    pub upload_rate: u64,
    pub total_size: u64,
    pub downloaded: u64,
    pub num_peers: u32,
    pub state: DownloadState,
    pub save_path: String,
    pub added_date: String,
    pub magnet: String,
    pub error: Option<String>,
}

pub struct TorrentSessionInner {
    pub downloads: HashMap<String, TorrentHandle>,
    pub settings: AppSettings,
}

#[derive(Clone)]
pub struct TorrentSession {
    pub inner: Arc<RwLock<TorrentSessionInner>>,
    api: Arc<Api>,
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
                let dht_dir = download_dir.join(".rqbit-dht");
                std::fs::create_dir_all(&dht_dir).ok();
                dht_cfg.persistence = Some(librqbit::dht::DhtPersistenceConfig {
                    dump_interval: None,
                    config_filename: Some(dht_dir.join("dht.json")),
                });
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

        let api = Arc::new(Api::new(session, None));

        let inner = Arc::new(RwLock::new(TorrentSessionInner {
            downloads: HashMap::new(),
            settings,
        }));

        Self { inner, api }
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

        let api = Arc::new(Api::new(session, None));
        let inner = Arc::new(RwLock::new(TorrentSessionInner {
            downloads: HashMap::new(),
            settings,
        }));

        Self { inner, api }
    }

    pub async fn add_download(&self, magnet: &str, save_path: &str) -> Result<String, String> {
        let hash = extract_hash_from_magnet(magnet)
            .unwrap_or_else(|| format!("{:x}", hash_fallback(magnet.as_bytes())));

        let resolved_path = resolve_save_path(save_path);
        std::fs::create_dir_all(&resolved_path).ok();
        let save_path_str = resolved_path.to_string_lossy().into_owned();

        eprintln!("[nyaland] add_download START: hash={hash}, magnet_len={}, magnet_start=\"{}\"",
            magnet.len(),
            &magnet[..magnet.len().min(80)]
        );

        let dht_stats = self.api.api_dht_stats();
        eprintln!("[nyaland] DHT stats before add: {:?}", dht_stats.as_ref().map(|s| (&s.id, s.outstanding_requests, s.routing_table_size)));

        {
            let mut inner = self.inner.write().await;
            if inner.downloads.contains_key(&hash) {
                eprintln!("[nyaland] add_download: torrent already exists, returning existing hash");
                return Ok(hash);
            }

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

        eprintln!("[nyaland] add_download: handle inserted, spawning api_add_torrent task...");

        let api = self.api.clone();
        let inner = self.inner.clone();
        let magnet_owned = magnet.to_string();
        let hash_clone = hash.clone();

        tokio::spawn(async move {
            eprintln!("[nyaland] spawned task START: calling api_add_torrent...");
            let add_opts = AddTorrentOptions {
                overwrite: true,
                output_folder: Some(save_path_str),
                ..Default::default()
            };

            eprintln!("[nyaland] api_add_torrent: about to call with magnet=\"{}\"",
                &magnet_owned[..magnet_owned.len().min(80)]
            );

            let result = tokio::time::timeout(
                std::time::Duration::from_secs(60),
                api.api_add_torrent(
                    AddTorrent::Url(Cow::Owned(magnet_owned.clone())),
                    Some(add_opts),
                ),
            ).await;

            match result {
                Ok(Ok(resp)) => {
                    eprintln!("[nyaland] api_add_torrent OK: id={:?}, output_folder={:?}",
                        resp.id, resp.output_folder);
                    if let Some(id) = resp.id {
                        let mut inner = inner.write().await;
                        if let Some(handle) = inner.downloads.get_mut(&hash_clone) {
                            handle.torrent_id = Some(id);
                            let details = api
                                .api_torrent_details(TorrentIdOrHash::Id(id))
                                .ok();
                            if let Some(d) = &details {
                                if let Some(ref n) = d.name {
                                    handle.name = n.clone();
                                }
                                handle.total_size = d.stats.as_ref().map(|s| s.total_bytes).unwrap_or(0);
                                eprintln!(
                                    "[nyaland] torrent resolved: name={:?}, id={id}, total_size={}",
                                    handle.name, handle.total_size
                                );
                            }
                        }
                    } else {
                        eprintln!("[nyaland] api_add_torrent: resp.id is None (ListOnly)");
                        let mut inner = inner.write().await;
                        if let Some(handle) = inner.downloads.get_mut(&hash_clone) {
                            handle.state = DownloadState::Error;
                            handle.error = Some("No torrent ID returned (list-only response)".to_string());
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("[nyaland] api_add_torrent FAILED: {e}");
                    let mut inner = inner.write().await;
                    if let Some(handle) = inner.downloads.get_mut(&hash_clone) {
                        handle.state = DownloadState::Error;
                        handle.error = Some(e.to_string());
                    }
                }
                Err(_) => {
                    eprintln!("[nyaland] api_add_torrent TIMEOUT after 60s");
                    let mut inner = inner.write().await;
                    if let Some(handle) = inner.downloads.get_mut(&hash_clone) {
                        handle.state = DownloadState::Error;
                        handle.error = Some("Timeout: magnet resolution took too long (60s)".to_string());
                    }
                }
            }
        });

        Ok(hash)
    }

    pub async fn get_downloads(&self) -> Vec<crate::types::DownloadStatus> {
        let mut inner = self.inner.write().await;

        let count = inner.downloads.len();
        eprintln!("[nyaland] get_downloads: {} entries", count);

        for (hash, handle) in inner.downloads.iter_mut() {
            if let Some(id) = handle.torrent_id {
                let tid = TorrentIdOrHash::Id(id);
                match self.api.session().get(tid) {
                    Some(mgr) => {
                        let stats = mgr.stats();
                        let old_progress = handle.progress;
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

                        let old_state = handle.state.clone();
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
                                librqbit::TorrentStatsState::Initializing => DownloadState::Connecting,
                                librqbit::TorrentStatsState::Error => DownloadState::Error,
                            }
                        };

                        if handle.state != old_state || (handle.progress - old_progress).abs() > 0.001 {
                            eprintln!("[nyaland] get_downloads: hash={hash} name={:?} state={:?}->{:?} progress={:.1}% dl_rate={} live={}",
                                handle.name, old_state, handle.state,
                                handle.progress * 100.0,
                                handle.download_rate,
                                stats.live.is_some(),
                            );
                        }
                    }
                    None => {
                        eprintln!("[nyaland] get_downloads: hash={hash} id={id} NOT FOUND in session (torrent_id={:?}, state={:?})",
                            handle.torrent_id, handle.state);
                    }
                }
            } else {
                eprintln!("[nyaland] get_downloads: hash={hash} no torrent_id yet (state={:?})", handle.state);
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
                self.api
                    .api_torrent_action_pause(TorrentIdOrHash::Id(id))
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
                self.api
                    .api_torrent_action_start(TorrentIdOrHash::Id(id))
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
                if delete_files {
                    self.api
                        .api_torrent_action_delete(TorrentIdOrHash::Id(id))
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    self.api
                        .api_torrent_action_forget(TorrentIdOrHash::Id(id))
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
            return Ok(true);
        }
        Err("Download not found".to_string())
    }
}

fn extract_hash_from_magnet(magnet: &str) -> Option<String> {
    let prefix = "xt=urn:btih:";
    let start = magnet.find(prefix)? + prefix.len();
    let rest = &magnet[start..];
    let end = rest.find('&').unwrap_or(rest.len());
    let hash = &rest[..end];
    if hash.len() == 40 {
        Some(hash.to_lowercase())
    } else {
        None
    }
}

fn truncate_magnet(magnet: &str) -> String {
    if let Some(name_start) = magnet.find("dn=") {
        let encoded = &magnet[name_start + 3..];
        let end = encoded.find('&').unwrap_or(encoded.len());
        let name = &encoded[..end];
        urlencoding::decode(name).unwrap_or_default().to_string()
    } else if magnet.len() > 60 {
        format!("{}...", &magnet[..60])
    } else {
        magnet.to_string()
    }
}

fn hash_fallback(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
