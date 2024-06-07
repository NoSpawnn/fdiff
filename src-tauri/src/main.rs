// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[derive(serde::Serialize)]
struct LineDiff {
    left_text: String,
    right_text: String,
    is_diff: bool,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn get_diff(left_text: &str, right_text: &str) -> Vec<LineDiff> {
    let mut line_diffs: Vec<LineDiff> = Vec::new();
    let mut left_text_lines = left_text.lines();
    let mut right_text_lines = right_text.lines();

    loop {
        match (left_text_lines.next(), right_text_lines.next()) {
            (Some(x), Some(y)) => line_diffs.push(LineDiff {
                left_text: x.to_string(),
                right_text: y.to_string(),
                is_diff: x != y,
            }),
            (Some(x), None) => line_diffs.push(LineDiff {
                left_text: x.to_string(),
                right_text: "".to_string(),
                is_diff: true,
            }),
            (None, Some(y)) => line_diffs.push(LineDiff {
                left_text: "".to_string(),
                right_text: y.to_string(),
                is_diff: true,
            }),
            (None, None) => break,
        }
    }

    return line_diffs;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // Only in dev mode
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_diff])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
