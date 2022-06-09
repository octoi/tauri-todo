#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod controller;
pub mod database;

use database::init_database;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::user::login_user,
            controller::user::sign_up_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
