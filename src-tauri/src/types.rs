use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::torrent::DownloadState;

pub fn resolve_save_path(path: &str) -> PathBuf {
    let path = path.trim();
    if path.is_empty() {
        return std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    }
    if path.starts_with("~/") || path == "~" {
        if let Some(home) = dirs::home_dir() {
            let relative = if path == "~" { "" } else { &path[2..] };
            return home.join(relative);
        }
    }
    PathBuf::from(path)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub page: Option<u64>,
    pub category: Option<String>,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDownloadParams {
    pub magnet: String,
    pub save_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadActionParams {
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveDownloadParams {
    pub hash: String,
    pub delete_files: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStatus {
    pub hash: String,
    pub name: String,
    pub progress: f32,
    pub download_rate: u64,
    pub upload_rate: u64,
    pub total_size: u64,
    pub downloaded: u64,
    pub num_peers: u32,
    pub state: DownloadState,
    pub save_path: String,
    pub added_date: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DownloadFilter {
    All,
    Downloading,
    Paused,
    Finished,
    Error,
}

impl DownloadFilter {
    pub fn matches(&self, state: &DownloadState) -> bool {
        match self {
            DownloadFilter::All => true,
            DownloadFilter::Downloading => {
                matches!(
                    state,
                    DownloadState::Downloading | DownloadState::Connecting | DownloadState::Queued
                )
            }
            DownloadFilter::Paused => matches!(state, DownloadState::Paused),
            DownloadFilter::Finished => matches!(state, DownloadState::Finished),
            DownloadFilter::Error => matches!(state, DownloadState::Error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDownloadsParams {
    pub filter: Option<DownloadFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadHistoryEntry {
    pub hash: String,
    pub name: String,
    pub magnet: String,
    pub save_path: String,
    pub total_size: u64,
    pub state: DownloadState,
    pub added_date: String,
    pub completed_date: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub save_path: String,
    pub nyaa_base_url: String,
    pub max_download_speed: i64,
    pub max_upload_speed: i64,
    pub max_connections: i32,
    pub max_active_downloads: i32,
    pub start_on_launch: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        let save_path = dirs::download_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("Nyaland");

        Self {
            save_path: save_path.to_string_lossy().to_string(),
            nyaa_base_url: "https://nyaa.si".to_string(),
            max_download_speed: 0,
            max_upload_speed: 0,
            max_connections: 200,
            max_active_downloads: 5,
            start_on_launch: true,
        }
    }
}
