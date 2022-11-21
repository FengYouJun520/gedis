#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gedis::command::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test_connection,
            connection,
            dis_connection,
            dis_connection_all
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
