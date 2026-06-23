use super::state::DownloadState;

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
    pub downloads: std::collections::HashMap<String, TorrentHandle>,
    pub settings: crate::types::AppSettings,
}
