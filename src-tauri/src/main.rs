// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn submit(text1: &str, text2: &str) -> String {
    println!("Text 1: {text1}");
    println!("Text 2: {text2}");
    format!("{text1}\n{text2}")
}

fn main() {


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![submit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
