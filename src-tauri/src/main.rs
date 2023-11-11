// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{Connection, Result};

fn main() -> Result<()> {

    let conn = match Connection::open("connecti.db") {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error opening connection: {}", e);
            return Err(e.into())
        }
    };

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            first_name   TEXT NOT NULL,
            last_name    TEXT NOT NULL,
            email        TEXT NOT NULL,
            phone_number TEXT NOT NULL
        )",
        [],
    ) {
        Ok(_) => println!("Table created successfully"),
        Err(e) => {
            println!("Error creating table: {}", e);
            return Err(e.into())
        }
    };


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, add_one])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
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