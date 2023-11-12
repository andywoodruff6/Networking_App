// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{Connection, Result};
mod modules;
use modules::{Relationship, Person, History};

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

// Create - to database
#[tauri::command]
fn create_person(person: Person) -> Result<()> {
    
    let conn: Connection = create_connection()?;

    let relationship = match person.relationship {
        Relationship::Friend => "Friend",
        Relationship::Work => "Work",
        Relationship::Hobby => "Hobby",
    };

    conn.execute(
        "INSERT INTO person
        (first_name, last_name, relationship, email, phone_number)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            //&person.id,
            &person.first_name,
            &person.last_name,
            relationship,
            &person.email,
            &person.phone_number
        ],
    )?; 
    println!("Created person: {:#?}", person);

    Ok(())
}

// Read - from database
#[tauri::command]
fn read_person(id: i32) -> Result<Person, rusqlite::Error> {
    let conn: Connection = create_connection()?;

    let mut stmt = conn.prepare("SELECT * FROM person WHERE id = ?1")?;
    let person_iter = stmt.query_map([id], |row| {
        Ok(Person {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            relationship: match row.get::<_, String>(3)?.as_str() {
                "Friend" => Relationship::Friend,
                "Work" => Relationship::Work,
                "Hobby" => Relationship::Hobby,
                _ => Relationship::Friend,
            },
            email: row.get(4)?,
            phone_number: row.get(5)?,
        })
    })?;

    let person = person_iter
        .map(|person| person.unwrap())
        .collect::<Vec<Person>>()
        .pop()
        .unwrap();
    println!("found person: {:#?}", person);
    Ok(person)
}

// Update - to database
#[tauri::command]
fn update_person(id: i32, person: Person) -> Result<(), rusqlite::Error> {
    let conn: Connection = create_connection()?;

    let relationship = match person.relationship {
        Relationship::Friend => "Friend",
        Relationship::Work => "Work",
        Relationship::Hobby => "Hobby",
    };

    conn.execute(
        "UPDATE person
        SET first_name = ?1, last_name = ?2, relationship = ?3, email = ?4, phone_number = ?5
        WHERE id = ?6",
        [
            &person.first_name,
            &person.last_name,
            relationship,
            &person.email,
            &person.phone_number,
            &id.to_string()
        ]
    )?;
    println!("Updated person with id: {}", id);
    Ok(())
}

// Delete - from database
#[tauri::command]
fn delete_person(id: i32) -> Result<(), rusqlite::Error> {
    let conn: Connection = create_connection()?;

    conn.execute(
        "DELETE FROM person
        WHERE id = ?1",
        [id]
    )?;
    println!("Deleted person with id: {}", id);
    Ok(())
}



pub fn create_connection() -> Result<Connection, rusqlite::Error> {
    match Connection::open("connecti.db") {
    Ok(connection) => {
        println!("Connection established");
        Ok(connection)
        },
    Err(e) => {
        println!("Error opening connection: {}", e);
        return Err(e.into())
        }
    }
}