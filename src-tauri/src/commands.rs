use tauri::State;

use crate::nyaa::NyaaClient;
use crate::torrent::TorrentSession;
use crate::types::*;

pub struct AppState {
    pub nyaa: NyaaClient,
    pub torrent: TorrentSession,
}

#[tauri::command]
pub async fn search(
    state: State<'_, AppState>,
    params: SearchParams,
) -> Result<String, String> {
    let result = state.nyaa.search(&params).await?;
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn view_torrent(
    state: State<'_, AppState>,
    id: u64,
) -> Result<String, String> {
    let detail = state.nyaa.view(id).await?;
    serde_json::to_string(&detail).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_download(
    state: State<'_, AppState>,
    params: AddDownloadParams,
) -> Result<String, String> {
    state.torrent.add_download(&params.magnet, &params.save_path).await
}

#[tauri::command]
pub async fn get_downloads(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let downloads = state.torrent.get_downloads().await;
    serde_json::to_string(&downloads).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause_download(
    state: State<'_, AppState>,
    params: DownloadActionParams,
) -> Result<bool, String> {
    state.torrent.pause_download(&params.hash).await
}

#[tauri::command]
pub async fn resume_download(
    state: State<'_, AppState>,
    params: DownloadActionParams,
) -> Result<bool, String> {
    state.torrent.resume_download(&params.hash).await
}

#[tauri::command]
pub async fn remove_download(
    state: State<'_, AppState>,
    params: RemoveDownloadParams,
) -> Result<bool, String> {
    state.torrent.remove_download(&params.hash, params.delete_files).await
}

#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let inner = state.torrent.inner.read().await;
    serde_json::to_string(&inner.settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<bool, String> {
    let mut inner = state.torrent.inner.write().await;
    inner.settings = settings;
    Ok(true)
}
