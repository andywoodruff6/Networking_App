import Database from "tauri-plugin-sql-api";

export async function checkAndCreateTables() {
    // load database connection
    const db = await Database.load("sqlite:test.db");

    // check person table
    await db.execute(
        "CREATE TABLE IF NOT EXISTS person (id INTEGER PRIMARY KEY, first_name TEXT, last_name TEXT, relationship TEXT, email TEXT, phone_number TEXT)"
    );
    // check history table
    await db.execute(
        "CREATE TABLE IF NOT EXISTS history (id INTEGER PRIMARY KEY, person_id INTEGER, date TEXT, topic TEXT, contact_platform TEXT)"
    );
    // maybe future check relationship table
}
export async function addPerson(first_name, last_name, relationship, email, phone_number) {
    // load database connection
    const db = await Database.load("sqlite:test.db");
    // add person
    await db.execute(
        'INSERT INTO person (first_name, last_name, relationship, email, phone_number) VALUES (?,?,?,?,?)',
        [first_name, last_name, relationship, email, phone_number]
    );
    console.log('Person added');
}
export async function getPeople() {
    // load database connection
    const db = await Database.load("sqlite:test.db");
    // get people
    const people = await db.select('SELECT * FROM person');
    return people;
}
export async function getPeopleByRelationship(relationship) {
    // load database connection
    const db = await Database.load("sqlite:test.db");
    // get people
    const people = await db.select('SELECT * FROM person WHERE relationship = $1', [relationship]);
    return people;
}
export async function addHistory(person_id, date, topic, contact_platform) {
    // load database connection
    const db = await Database.load("sqlite:test.db");
    // add history
    await db.execute(
        'INSERT INTO history (person_id, date, topic, contact_platform) VALUES (?,?,?,?)',
        [person_id, date, topic, contact_platform]
    );
    console.log('History added');
}
export async function getHistory(id) {
    // load database connection
    const db = await Database.load("sqlite:test.db");
    // get history
    const history = await db.select('SELECT * FROM history WHERE person_id = $1', [id]);
    return history;
}
