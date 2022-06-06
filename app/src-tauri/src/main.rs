#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;

use database::init_database;

fn main() {
    match init_database("sample.db") {
        Ok(_) => println!("[+] DATABASE CONNECTED"),
        Err(_) => println!("[-] FAILED TO CONNECT DATABASE"),
    };

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
