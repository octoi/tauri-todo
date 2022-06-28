#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;
mod database;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::todo::create_todo,
            controller::todo::read_todos,
            controller::todo::update_todo,
            controller::todo::delete_todo,
        ])
        .run(context)
        .expect("error while running tauri application");
}
