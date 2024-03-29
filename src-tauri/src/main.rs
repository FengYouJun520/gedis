#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::Local;
use gedis::command::*;
use gedis::RedisState;
use tauri::Manager;
use tracing::Level;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};
use window_shadows::set_shadow;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_timer(LocalTimer)
        .init();
}

fn main() {
    init_tracing_subscriber();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            test_connection,
            connection,
            is_connection,
            ping,
            change_db,
            dis_connection,
            dis_connection_all,
            get_key_type,
            get_info,
            get_logs,
            clear_logs,
            get_key_detail,
            del_key,
            del_match_keys,
            del_key_by_value,
            clear_keys,
            get_keys_by_db,
            get_key_info,
            rename_key,
            set_key,
            set_key_ttl,
            terminal
        ])
        .manage(RedisState::default())
        .manage(History::default())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
