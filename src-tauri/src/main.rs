// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod modules;
use modules::{Relationship, Person, History};

fn main() {

    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS person (
    //         id           INTEGER PRIMARY KEY,
    //         first_name   TEXT NOT NULL,
    //         last_name    TEXT NOT NULL,
    //         relationship TEXT NOT NULL,
    //         email        TEXT NOT NULL,
    //         phone_number TEXT NOT NULL
    //     )",
    //     [],
    // )?;

    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS history (
    //         id               INTEGER PRIMARY KEY,
    //         person_id        INTEGER NOT NULL,
    //         date             TEXT NOT NULL,
    //         topic            TEXT NOT NULL,
    //         contact_platform TEXT NOT NULL,
    //         FOREIGN KEY (person_id) REFERENCES person (id)
    //     )",
    //     [],
    // )?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, add_one])
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_one(number: i32) -> i32 {
    println!("Hello from Rust! {}", number);
    return number + 1;
}