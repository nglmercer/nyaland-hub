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
    if cfg!(target_os = "android") {
        return PathBuf::from("/storage/emulated/0/Download/Nyaland")
            .join(".nyaland_settings.json");
    }
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

#[cfg(target_os = "android")]
fn get_mime_type(path: &str) -> String {
    match path.rsplit('.').next().unwrap_or("") {
        "mkv" => "video/x-matroska",
        "mp4" => "video/mp4",
        "avi" => "video/x-msvideo",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "flv" => "video/x-flv",
        "wmv" => "video/x-ms-wmv",
        "m4v" => "video/x-m4v",
        "ts" => "video/mp2t",
        "rmvb" => "application/vnd.rn-realmedia-vbr",
        "mp3" => "audio/mpeg",
        "flac" => "audio/flac",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        _ => "*/*",
    }
    .to_string()
}

#[tauri::command]
pub async fn open_file_with_shell(file_path: String) -> Result<(), String> {
    let path = std::path::Path::new(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {file_path}"));
    }

    #[cfg(target_os = "android")]
    {
        let mime = get_mime_type(&file_path);
        let content_uri = file_path_to_content_uri(&file_path);
        std::process::Command::new("/system/bin/am")
            .args([
                "start",
                "-a",
                "android.intent.action.VIEW",
                "-d",
                &content_uri,
                "-t",
                &mime,
                "--grant-read-uri-permission",
            ])
            .output()
            .map_err(|e| format!("Failed to launch player: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &file_path])
            .spawn()
            .map_err(|e| format!("Failed to open file: {e}"))?;
        Ok(())
    }
}

#[tauri::command]
pub async fn open_folder_with_shell(folder_path: String) -> Result<(), String> {
    let path = std::path::Path::new(&folder_path);
    if !path.exists() {
        return Err(format!("Folder not found: {folder_path}"));
    }

    #[cfg(target_os = "android")]
    {
        let content_uri = file_path_to_content_uri(&folder_path);
        std::process::Command::new("/system/bin/am")
            .args([
                "start",
                "-a",
                "android.intent.action.VIEW",
                "-d",
                &content_uri,
                "-t",
                "vnd.android.document/directory",
                "--grant-read-uri-permission",
            ])
            .output()
            .map_err(|e| format!("Failed to open folder: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {e}"))?;
        Ok(())
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&folder_path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {e}"))?;
        Ok(())
    }
}

#[cfg(target_os = "android")]
fn file_path_to_content_uri(file_path: &str) -> String {
    let prefix = "/storage/emulated/0/";
    if let Some(relative) = file_path.strip_prefix(prefix) {
        format!(
            "content://com.nyaland.desktop.fileprovider/external_files/{}",
            relative.trim_start_matches('/')
        )
    } else {
        format!("file://{file_path}")
    }
}
