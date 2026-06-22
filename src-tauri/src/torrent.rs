use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::types::AppSettings;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DownloadState {
    Queued,
    Downloading,
    Paused,
    Finished,
    Error,
    Moving,
}

pub struct TorrentHandle {
    pub name: String,
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
}

#[derive(Clone)]
pub struct TorrentSession {
    pub downloads: Arc<RwLock<HashMap<String, TorrentHandle>>>,
    pub settings: Arc<RwLock<AppSettings>>,
}

impl TorrentSession {
    pub fn new(settings: AppSettings) -> Self {
        let downloads = Arc::new(RwLock::new(HashMap::new()));
        let settings = Arc::new(RwLock::new(settings));
        Self { downloads, settings }
    }

    pub async fn add_download(&self, magnet: &str, save_path: &str) -> Result<String, String> {
        let hash = extract_hash_from_magnet(magnet)
            .unwrap_or_else(|| format!("{:x}", md5_simple(magnet.as_bytes())));

        let handle = TorrentHandle {
            name: truncate_magnet(magnet),
            progress: 0.0,
            download_rate: 0,
            upload_rate: 0,
            total_size: 0,
            downloaded: 0,
            num_peers: 0,
            state: DownloadState::Queued,
            save_path: save_path.to_string(),
            added_date: chrono::Utc::now().to_rfc3339(),
            magnet: magnet.to_string(),
        };

        let mut downloads = self.downloads.write().await;
        downloads.insert(hash.clone(), handle);

        Ok(hash)
    }

    pub async fn get_downloads(&self) -> Vec<crate::types::DownloadStatus> {
        let downloads = self.downloads.read().await;
        downloads
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
        let mut downloads = self.downloads.write().await;
        if let Some(handle) = downloads.get_mut(hash) {
            handle.state = DownloadState::Paused;
            handle.download_rate = 0;
            return Ok(true);
        }
        Err("Download not found".to_string())
    }

    pub async fn resume_download(&self, hash: &str) -> Result<bool, String> {
        let mut downloads = self.downloads.write().await;
        if let Some(handle) = downloads.get_mut(hash) {
            handle.state = DownloadState::Downloading;
            return Ok(true);
        }
        Err("Download not found".to_string())
    }

    pub async fn remove_download(&self, hash: &str, _delete_files: bool) -> Result<bool, String> {
        let mut downloads = self.downloads.write().await;
        if downloads.remove(hash).is_some() {
            return Ok(true);
        }
        Err("Download not found".to_string())
    }

    pub async fn simulate_progress(&self) {
        let mut downloads = self.downloads.write().await;
        for (_hash, handle) in downloads.iter_mut() {
            if handle.state == DownloadState::Downloading && handle.progress < 1.0 {
                let increment = (rand_f32() * 0.02).min(0.02);
                handle.progress = (handle.progress + increment).min(1.0);
                handle.download_rate = (rand_u64() % 10_000_000) + 500_000;
                handle.upload_rate = (rand_u64() % 2_000_000) + 100_000;
                handle.num_peers = (rand_u32() % 50) + 5;

                if handle.total_size == 0 {
                    handle.total_size = 700_000_000 + (rand_u64() % 3_300_000_000);
                }
                handle.downloaded = (handle.progress * handle.total_size as f32) as u64;

                if handle.progress >= 1.0 {
                    handle.state = DownloadState::Finished;
                    handle.download_rate = 0;
                    handle.upload_rate = 0;
                }
            }
        }
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

fn rand_f32() -> f32 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let s = RandomState::new();
    let mut hasher = s.build_hasher();
    hasher.write_u64(0);
    (hasher.finish() as f32) / (u64::MAX as f32)
}

fn rand_u64() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let s = RandomState::new();
    let mut hasher = s.build_hasher();
    hasher.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64);
    hasher.finish()
}

fn rand_u32() -> u32 {
    rand_u64() as u32
}

fn md5_simple(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
