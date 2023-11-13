// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod modules;
mod crud;

use rusqlite::{Connection, Result};
use modules::{Relationship, Person, History};
use crud::{create_connection, create_person, read_person, delete_person, update_person};

fn main() -> Result<()> {

    let conn: Connection = create_connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id           INTEGER PRIMARY KEY,
            first_name   TEXT NOT NULL,
            last_name    TEXT NOT NULL,
            relationship TEXT NOT NULL,
            email        TEXT NOT NULL,
            phone_number TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            id               INTEGER PRIMARY KEY,
            person_id        INTEGER NOT NULL,
            date             TEXT NOT NULL,
            topic            TEXT NOT NULL,
            contact_platform TEXT NOT NULL,
            FOREIGN KEY (person_id) REFERENCES person (id)
        )",
        [],
    )?;

    let test: Person = Person {
        id: 1,
        first_name: "first".to_string(),
        last_name: "last".to_string(),
        relationship: Relationship::Friend,
        email: "test@test.com".to_string(),
        phone_number: "1234567890".to_string()
    };
    create_person(test)?;
    read_person(1)?;

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