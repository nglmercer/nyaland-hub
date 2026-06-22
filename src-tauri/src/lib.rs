pub mod commands;
pub mod nyaa;
pub mod torrent;
pub mod types;

pub use commands::AppState;

use nyaa::NyaaClient;
use torrent::TorrentSession;
use types::AppSettings;

pub async fn run() {
    let settings = AppSettings::default();
    let nyaa = NyaaClient::new(&settings.nyaa_base_url);
    let torrent = TorrentSession::new(settings).await;

    let state = AppState { nyaa, torrent };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
