use rusqlite::{params, Connection};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

pub fn database_test(database_location: String) -> String {
    let connection = Connection::open(database_location).unwrap();

    connection.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
        params![],
    ).unwrap();

    connection.execute(
        "INSERT INTO person (name) VALUES (?1)",
        params!["Ada Lovelace"],
    ).unwrap();

    let mut stmt = connection.prepare("SELECT id, name FROM person").unwrap();
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        })
    }).unwrap();

    let person = person_iter.last().unwrap().unwrap();

    format!("{:?}", person)
}
