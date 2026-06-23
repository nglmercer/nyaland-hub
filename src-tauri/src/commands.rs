use std::path::PathBuf;

use tauri::State;

use crate::nyaa::NyaaClient;
use crate::torrent::TorrentSession;
use crate::types::*;

pub struct AppState {
    pub nyaa: NyaaClient,
    pub torrent: TorrentSession,
    pub _rt: tokio::runtime::Runtime,
}

fn settings_file_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("nyaland")
        .join("settings.json")
}

pub fn load_settings_from_disk() -> AppSettings {
    let path = settings_file_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        if let Ok(mut settings) = serde_json::from_str::<AppSettings>(&data) {
            settings.save_path = resolve_save_path(&settings.save_path)
                .to_string_lossy()
                .to_string();
            return settings;
        }
    }
    AppSettings::default()
}

#[tauri::command]
pub async fn search(state: State<'_, AppState>, params: SearchParams) -> Result<String, String> {
    let result = state.nyaa.search(&params).await?;
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn view_torrent(state: State<'_, AppState>, id: u64) -> Result<String, String> {
    let detail = state.nyaa.view(id).await?;
    serde_json::to_string(&detail).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_download(
    state: State<'_, AppState>,
    params: AddDownloadParams,
) -> Result<String, String> {
    state
        .torrent
        .add_download(&params.magnet, &params.save_path)
        .await
}

#[tauri::command]
pub async fn get_downloads(state: State<'_, AppState>) -> Result<String, String> {
    let downloads = state.torrent.get_downloads().await;
    serde_json::to_string(&downloads).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_downloads_filtered(
    state: State<'_, AppState>,
    params: GetDownloadsParams,
) -> Result<String, String> {
    let filter = params.filter.unwrap_or(DownloadFilter::All);
    let downloads = state.torrent.get_downloads_filtered(&filter).await;
    serde_json::to_string(&downloads).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_download_history(state: State<'_, AppState>) -> Result<String, String> {
    let history = state.torrent.get_history().await;
    serde_json::to_string(&history).map_err(|e| e.to_string())
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
    state
        .torrent
        .remove_download(&params.hash, params.delete_files)
        .await
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<String, String> {
    let settings = load_settings_from_disk();
    let mut inner = state.torrent.inner.write().await;
    inner.settings = settings.clone();
    serde_json::to_string(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    mut settings: AppSettings,
) -> Result<bool, String> {
    settings.save_path = resolve_save_path(&settings.save_path)
        .to_string_lossy()
        .to_string();
    let path = settings_file_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;

    let mut inner = state.torrent.inner.write().await;
    inner.settings = settings;
    Ok(true)
}

#[tauri::command]
pub async fn detect_media_files(path: String) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(&path).map_err(|e| e.to_string())?;
    let media_exts = [
        "mkv", "mp4", "avi", "webm", "mov", "flv", "wmv", "m4v", "ts", "rmvb",
    ];
    let mut files = Vec::new();

    for entry in entries.flatten() {
        if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
            if media_exts.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                files.push(entry.path().to_string_lossy().to_string());
            }
        }
    }

    files.sort();
    Ok(files)
}

#[tauri::command]
pub async fn detect_media_files_recursive(path: String) -> Result<Vec<String>, String> {
    let media_exts = [
        "mkv", "mp4", "avi", "webm", "mov", "flv", "wmv", "m4v", "ts", "rmvb",
    ];
    let mut files = Vec::new();

    fn walk_dir(
        dir: &std::path::Path,
        media_exts: &[&str],
        files: &mut Vec<String>,
    ) -> Result<(), String> {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path, media_exts, files)?;
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if media_exts.iter().any(|e| ext.eq_ignore_ascii_case(e)) {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        Ok(())
    }

    let root = std::path::Path::new(&path);
    walk_dir(root, &media_exts, &mut files)?;
    files.sort();
    Ok(files)
}
