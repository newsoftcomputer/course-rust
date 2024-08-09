
#[macro_use] extern crate rocket;
use rusqlite::{params, Connection, Result};

// Structs
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}


#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

fn createdb() -> Result<()> {

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id INTEGER KEY,
            name TEXT NOT NULL,
            data BLOB
            )", 
        (),
    )?;

    let me = Person {
        id: 0,
        name: "Andres".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)", 
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())

}