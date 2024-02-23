#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![process])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn process(input: &str) -> String {
    1.to_string()
}