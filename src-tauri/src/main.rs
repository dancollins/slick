#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;

mod mesh;

#[tauri::command]
fn load_file(path: &str) -> mesh::Mesh {
    mesh::load(Path::new(path))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
