pub mod commands;
pub mod nyaa;
pub mod torrent;
pub mod types;

pub use commands::AppState;

use nyaa::NyaaClient;
use torrent::TorrentSession;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let settings = commands::load_settings_from_disk();
    let nyaa = NyaaClient::new(&settings.nyaa_base_url);

    let torrent = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt.block_on(TorrentSession::new(settings)),
        Err(e) => {
            eprintln!("Failed to create tokio runtime: {e}");
            TorrentSession::new_fallback(settings)
        }
    };

    let state = AppState { nyaa, torrent };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::search,
            commands::view_torrent,
            commands::add_download,
            commands::get_downloads,
            commands::pause_download,
            commands::resume_download,
            commands::remove_download,
            commands::get_settings,
            commands::save_settings,
            commands::detect_media_files,
            commands::detect_media_files_recursive,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
