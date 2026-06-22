use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use librqbit::api::TorrentIdOrHash;
use librqbit::{
    AddTorrent, AddTorrentOptions, Api, Session, SessionOptions, SessionPersistenceConfig,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::types::AppSettings;

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
        let download_dir = PathBuf::from(&settings.save_path);
        std::fs::create_dir_all(&download_dir).ok();

        let session_opts = SessionOptions {
            persistence: Some(SessionPersistenceConfig::Json {
                folder: Some(download_dir.join(".rqbit-session")),
            }),
            ..Default::default()
        };

        let session = Session::new_with_opts(download_dir.clone(), session_opts)
            .await
            .expect("Failed to create session");

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

        let add_opts = AddTorrentOptions {
            overwrite: true,
            ..Default::default()
        };

        match self
            .api
            .api_add_torrent(
                AddTorrent::Url(Cow::Owned(magnet.to_string())),
                Some(add_opts),
            )
            .await
        {
            Ok(resp) => {
                let id = resp.id.ok_or("No torrent ID returned")?;

                let mut inner = self.inner.write().await;
                let info = self.api.api_torrent_details(TorrentIdOrHash::Id(id)).ok();
                let name = info
                    .as_ref()
                    .and_then(|d| d.name.clone())
                    .unwrap_or_else(|| truncate_magnet(magnet));

                let handle = TorrentHandle {
                    name,
                    torrent_id: Some(id),
                    progress: 0.0,
                    download_rate: 0,
                    upload_rate: 0,
                    total_size: 0,
                    downloaded: 0,
                    num_peers: 0,
                    state: DownloadState::Connecting,
                    save_path: save_path.to_string(),
                    added_date: chrono::Utc::now().to_rfc3339(),
                    magnet: magnet.to_string(),
                    error: None,
                };

                inner.downloads.insert(hash.clone(), handle);
                Ok(hash)
            }
            Err(e) => {
                let handle = TorrentHandle {
                    name: truncate_magnet(magnet),
                    torrent_id: None,
                    progress: 0.0,
                    download_rate: 0,
                    upload_rate: 0,
                    total_size: 0,
                    downloaded: 0,
                    num_peers: 0,
                    state: DownloadState::Error,
                    save_path: save_path.to_string(),
                    added_date: chrono::Utc::now().to_rfc3339(),
                    magnet: magnet.to_string(),
                    error: Some(e.to_string()),
                };

                let mut inner = self.inner.write().await;
                inner.downloads.insert(hash.clone(), handle);
                Ok(hash)
            }
        }
    }

    pub async fn get_downloads(&self) -> Vec<crate::types::DownloadStatus> {
        let mut inner = self.inner.write().await;

        for (_hash, handle) in inner.downloads.iter_mut() {
            if let Some(id) = handle.torrent_id {
                let tid = TorrentIdOrHash::Id(id);
                if let Some(mgr) = self.api.session().get(tid) {
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
                            librqbit::TorrentStatsState::Initializing => DownloadState::Connecting,
                            librqbit::TorrentStatsState::Error => DownloadState::Error,
                        }
                    };
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
