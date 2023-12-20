import Database from "tauri-plugin-sql-api";
const PROD_DB = "sqlite:prod.db";

export async function checkAndCreateTables() {
    // load database connection
    const db = await Database.load(PROD_DB);

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
// person functions
// -------------------------------------------
export async function addPerson(first_name, last_name, relationship, email, phone_number) {
    // load database connection
    const db = await Database.load(PROD_DB);
    // add person
    await db.execute(
        'INSERT INTO person (first_name, last_name, relationship, email, phone_number) VALUES (?,?,?,?,?)',
        [first_name, last_name, relationship, email, phone_number]
    );
    console.log('Person added');
}
export async function updatePerson(id, first_name, last_name, relationship, email, phone_number) {
    // load database connection
    const db = await Database.load(PROD_DB);
    // update person
    await db.execute(
        'UPDATE person SET first_name = ?, last_name = ?, relationship = ?, email = ?, phone_number = ? WHERE id = ?',
        [first_name, last_name, relationship, email, phone_number, id]
    );
    console.log('Person updated');
}
export async function deletePerson(id) {
    // load database connection
    const db = await Database.load(PROD_DB);
    // delete person
    await db.execute(
        'DELETE FROM person WHERE id = ?',
        [id]
    );
    console.log('Person deleted');
}
export async function getPeople() {
    // load database connection
    const db = await Database.load(PROD_DB);
    // get people
    const people = await db.select('SELECT * FROM person');   
    return people;
}
export async function getPeopleByRelationship(relationship) {
    // load database connection
    const db = await Database.load(PROD_DB);
    // get people
    const people = await db.select('SELECT * FROM person WHERE relationship = $1', [relationship]);
    return people;
}
// history functions
// -------------------------------------------
export async function addHistory(person_id, date, topic, contact_platform) {
    // load database connection
    const db = await Database.load(PROD_DB);
    // add history
    await db.execute(
        'INSERT INTO history (person_id, date, topic, contact_platform) VALUES (?,?,?,?)',
        [person_id, date, topic, contact_platform]
    );
    console.log('History added');
}
export async function getHistoryById(id) {
    // load database connection
    const db = await Database.load(PROD_DB);
    // get history
    const history = await db.select('SELECT * FROM history WHERE person_id = $1 ORDER BY date DESC', [id]);
    // console.log("before:",history);
    history.forEach(item => {
        item.date = timestampToDate(item.date);
    });
    // console.log("after:",history);
    return history;
}

// calendar functions
// -------------------------------------------
export async function testDBCalendar() {
    // load database connection
    const db = await Database.load(PROD_DB);
    // create join table
    const calendarArray = await db.select(
    'SELECT person.id, person.first_name, person.last_name, person.relationship, MAX(history.date) as max_date, history.topic, history.contact_platform FROM person INNER JOIN history ON person.id = history.person_id GROUP BY person.id')
    console.log(calendarArray);

    calendarArray.forEach(item => {
        switch (item.relationship) {
            case 'friend':
                item.max_date = item.max_date + (1000 * 60 * 60 * 24 * 7 * 1) // 1 week
                // console.log("friend triggered")
                break;
            case 'hobby':
                item.max_date = item.max_date + (1000 * 60 * 60 * 24 * 7 * 4) // 4 weeks
                // console.log("hobby triggered")
                break;
            case 'work':
                item.max_date = item.max_date + (1000 * 60 * 60 * 24 * 7 * 13) // 13 weeks
                // console.log("work triggered")
                break;
            default:
                item.max_date = item.max_date + (1000 *  60 * 60 * 24 * 7 * 4) // 4 weeks
                // console.log("default triggered")
            }
    })
    // need to sort the Array by max_date
    calendarArray.sort((a, b) => a.max_date - b.max_date);

    calendarArray.forEach(item => {
        item.max_date = timestampToDate(item.max_date);
    })

    return calendarArray;
}
export async function calendarByPosition(position) {
    // load database connection
    const db = await Database.load(PROD_DB);
    // create join table
    const calendarArray = await db.select(
    'SELECT person.id, person.first_name, person.last_name, person.relationship, MAX(history.date) as max_date, history.topic, history.contact_platform FROM person INNER JOIN history ON person.id = history.person_id GROUP BY person.id')
    console.log(calendarArray);

    calendarArray.forEach(item => {
        switch (item.relationship) {
            case 'friend':
                item.max_date = item.max_date + (1000 * 60 * 60 * 24 * 7 * 1) // 1 week
                // console.log("friend triggered")
                break;
            case 'hobby':
                item.max_date = item.max_date + (1000 * 60 * 60 * 24 * 7 * 4) // 4 weeks
                // console.log("hobby triggered")
                break;
            case 'work':
                item.max_date = item.max_date + (1000 * 60 * 60 * 24 * 7 * 13) // 13 weeks
                // console.log("work triggered")
                break;
            default:
                item.max_date = item.max_date + (1000 *  60 * 60 * 24 * 7 * 4) // 4 weeks
                // console.log("default triggered")
            }
    })
    // need to sort the Array by max_date
    calendarArray.sort((a, b) => a.max_date - b.max_date);

    const calendarObjectByPosition = calendarArray[position]
    console.log("calendar object by position", calendarObjectByPosition)
 
    calendarObjectByPosition.max_date = timestampToDate(calendarObjectByPosition.max_date);

    return calendarObjectByPosition;
}


// helper functions
function timestampToDate(timestamp) {
    const date = new Date(timestamp);
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');
    const year = date.getFullYear();
    return `${month}-${day}-${year}`;
}