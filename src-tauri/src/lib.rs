pub mod commands;
pub mod nyaa;
pub mod torrent;
pub mod types;

pub use commands::AppState;

use nyaa::NyaaClient;
use torrent::TorrentSession;
use types::AppSettings;

pub fn run() {
    let settings = AppSettings::default();
    let nyaa = NyaaClient::new(&settings.nyaa_base_url);
    let torrent = TorrentSession::new(settings);

    let state = AppState {
        nyaa,
        torrent: torrent.clone(),
    };

    let rt = tokio::runtime::Runtime::new().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(move |_app| {
            let session = torrent;
            std::thread::spawn(move || {
                rt.block_on(async move {
                    loop {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        session.simulate_progress().await;
                    }
                });
            });
            Ok(())
        })
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
            commands::simulate_progress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
