#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(nyaland_lib::run());
}
