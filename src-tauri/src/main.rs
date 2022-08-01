#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

#[tauri::command]
fn parse_file(path: String) -> String {
    // Read the file and return it's contents
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn measure_perfomance() -> Vec<u64> {
    let mut sss: u64 = 0;
    let mut vec = Vec::with_capacity(2);
    // Start time
    let start = std::time::Instant::now();

    for i in 0..10000000 {
        if i % 2 == 0 {
            sss += i;
        }
        if i % 3 == 0 && i * i < 500 || i / 2 == 0 {
            sss *= 2;
        }
        if i % 5 == 0 && i * i < 500 || i / 2 == 0 {
            sss /= 2;
        }
    }
    // End time
    let end = std::time::Instant::now();
    vec.push(sss);
    // Calculate the difference in time
    let duration = end.duration_since(start);
    // Return the duration in milliseconds
    vec.push(duration.as_millis() as u64);
    vec
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            parse_file,
            measure_perfomance
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
