use rusqlite::{Connection, Result};
use tauri::utils::platform;
use crate::modules::{Relationship, Person, History};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////
// PERSON CRUD
///////////////////////////////////////////////////////////////////
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

// Create - to database
#[tauri::command]
pub fn create_person(person: Person) -> Result<()> {
    
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
pub fn read_person(id: i32) -> Result<Person, rusqlite::Error> {
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

    let person: Person = person_iter
        .map(|person| person.unwrap())
        .collect::<Vec<Person>>()
        .pop()
        .unwrap();
    println!("found person: {:#?}", person);
    Ok(person)
}

// Update - to database
#[tauri::command]
pub fn update_person(id: i32, person: Person) -> Result<(), rusqlite::Error> {
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
pub fn delete_person(id: i32) -> Result<(), rusqlite::Error> {
    let conn: Connection = create_connection()?;

    conn.execute(
        "DELETE FROM person
        WHERE id = ?1",
        [id]
    )?;
    println!("Deleted person with id: {}", id);
    Ok(())
}


///////////////////////////////////////////////////////////////////
// HISTORY CRUD
///////////////////////////////////////////////////////////////////

// Create - to database
#[tauri::command]
pub fn create_history(person_id: i32, topic: String, platform: String) -> Result<(), rusqlite::Error> {
    let conn: Connection = create_connection()?;

    let date: String = Utc::now().to_rfc3339();
    let person_id_string: String = person_id.to_string();

    conn.execute(
        "INSERT INTO history
        (person_id, date, topic, contact_platform)
        VALUES (?1, ?2, ?3, ?4)",
        [
            &person_id_string,
            &date,
            &topic,
            &platform
        ],
    )?; 
    println!("Created history for person with id: {}", person_id);

    Ok(())
}

// Read - from database
#[tauri::command]
pub fn read_history(id: i32) -> Result<History, rusqlite::Error> {
    let conn: Connection = create_connection()?;

    let mut stmt = conn.prepare("SELECT * FROM history WHERE id = ?1")?;
    let history_iter = stmt.query_map([id], |row| {
        Ok(History {
            id: row.get(0)?,
            person_id: row.get(1)?,
            date: row.get(2)?,
            topic : row.get(3)?,
            contact_platform: row.get(4)?,
        })
    })?;

    let history: History = history_iter
        .map(|history| history.unwrap())
        .collect::<Vec<History>>()
        .pop()
        .unwrap();
    println!("found history: {:#?}", history);
    Ok(history)
}

// Update - to database
#[tauri::command]
pub fn update_history(id: i32, topic: String, platform: String) -> Result<(), rusqlite::Error> {
    let conn: Connection = create_connection()?;

    let date: String = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE history
        SET date = ?1, topic = ?2, contact_platform = ?3
        WHERE id = ?4",
        [
            &date,
            &topic,
            &platform,
            &id.to_string()
        ]
    )?;
    println!("Updated history: {}", id);
    Ok(())
}

// Delete - from database
#[tauri::command]
pub fn delete_history(id: i32) -> Result<(), rusqlite::Error> {
    let conn: Connection = create_connection()?;

    conn.execute(
        "DELETE FROM history
        WHERE id = ?1",
        [id]
    )?;
    println!("Deleted history with id: {}", id);
    Ok(())
}

