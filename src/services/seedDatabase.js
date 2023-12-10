import Database from "tauri-plugin-sql-api";
import { addPerson, addHistory } from "./database";

export async function seedDatabase() {
    console.log('seeding database');
    const db = await Database.load("sqlite:test.db")

    //--------------------
    // drop tables
    await db.execute("DROP TABLE IF EXISTS person");
    await db.execute("DROP TABLE IF EXISTS history");

    //--------------------
    // create tables
    // check person table
    await db.execute(
        "CREATE TABLE IF NOT EXISTS person (id INTEGER PRIMARY KEY, first_name TEXT, last_name TEXT, relationship TEXT, email TEXT, phone_number TEXT)"
    );
    // check history table
    await db.execute(
        "CREATE TABLE IF NOT EXISTS history (id INTEGER PRIMARY KEY, person_id INTEGER, date INTEGER, topic TEXT, contact_platform TEXT)"
    );

    //--------------------
    // add people
    await addPerson('Alice', 'Smith', 'friend', "alice@email.com", "1234");
    await addPerson('Bob', 'Wood', 'work', "bob@email.com", "5678");
    await addPerson('Charlie', 'Brown', 'hobby', "charlie@email.com", "12345678");
    await addPerson('Dave', 'Jones', 'friend', "dave@email.com", "9876");
    await addPerson('Eve', 'Taylor', 'work', "eve@email.com", "4321");

    //--------------------
    // add history alice
    await addHistory(0, 1702182224, 'coffee', 'zoom');
    await addHistory(0, 1702172224, 'beer', 'email');
    await addHistory(0, 1702162224, 'pizza', 'text');
    await addHistory(0, 1702152224, 'tea', 'zoom');
    await addHistory(0, 1702142224, 'wine', 'zoom');

    //--------------------
    // add history bob
    await addHistory(1, 1702182224, 'coffee', 'zoom');
    await addHistory(1, 1702172224, 'beer', 'email');
    await addHistory(1, 1702162224, 'pizza', 'text');
    await addHistory(1, 1702152224, 'tea', 'zoom');
    await addHistory(1, 1702142224, 'wine', 'zoom');

    //--------------------
    // add history charlie
    await addHistory(2, 1702182224, 'coffee', 'zoom');
    await addHistory(2, 1702172224, 'beer', 'email');
    await addHistory(2, 1702162224, 'pizza', 'text');
    await addHistory(2, 1702152224, 'tea', 'zoom');
    await addHistory(2, 1702142224, 'wine', 'zoom');

    //--------------------
    // add history dave
    await addHistory(3, 1702182224, 'coffee', 'zoom');
    await addHistory(3, 1702172224, 'beer', 'email');
    await addHistory(3, 1702162224, 'pizza', 'text');
    await addHistory(3, 1702152224, 'tea', 'zoom');
    await addHistory(3, 1702142224, 'wine', 'zoom');

    //--------------------
    // add history eve
    await addHistory(4, 1702182224, 'coffee', 'zoom');
    await addHistory(4, 1702172224, 'beer', 'email');
    await addHistory(4, 1702162224, 'pizza', 'text');
    await addHistory(4, 1702152224, 'tea', 'zoom');
    await addHistory(4, 1702142224, 'wine', 'zoom');

    //--------------------
    console.log('done seeding database');
}