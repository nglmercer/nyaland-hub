use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::torrent::DownloadState;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DownloadHistory {
    pub entries: Vec<DownloadHistoryEntry>,
}

impl DownloadHistory {
    pub fn load(path: &Path) -> Self {
        if let Ok(data) = std::fs::read_to_string(path) {
            if let Ok(history) = serde_json::from_str::<Self>(&data) {
                return history;
            }
        }
        Self::default()
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn upsert(&mut self, entry: DownloadHistoryEntry) {
        if let Some(existing) = self.entries.iter_mut().find(|e| e.hash == entry.hash) {
            *existing = entry;
        } else {
            self.entries.push(entry);
        }
    }

    pub fn remove(&mut self, hash: &str) -> Option<DownloadHistoryEntry> {
        if let Some(pos) = self.entries.iter().position(|e| e.hash == hash) {
            Some(self.entries.remove(pos))
        } else {
            None
        }
    }

    pub fn get(&self, hash: &str) -> Option<&DownloadHistoryEntry> {
        self.entries.iter().find(|e| e.hash == hash)
    }

    pub fn all(&self) -> &[DownloadHistoryEntry] {
        &self.entries
    }

    pub fn finished(&self) -> Vec<&DownloadHistoryEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(e.state, DownloadState::Finished))
            .collect()
    }
}

pub fn history_path() -> PathBuf {
    if cfg!(target_os = "android") {
        return PathBuf::from("/data/data/com.nyaland.desktop/files").join("download_history.json");
    }
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("nyaland")
        .join("download_history.json")
}
