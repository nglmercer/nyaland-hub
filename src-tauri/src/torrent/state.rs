use serde::{Deserialize, Serialize};

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
