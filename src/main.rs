use std::env;
use std::path::Path;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Monster {
    name: String,
    physical: i32,
    race: String,
    condition: String,
    rank: String,
    age: i32,
}

fn main() -> Result<()> {


    let conn = Connection::open(Path::new(&env::var("PROGRAMDATA").unwrap()).join("TakmaruApp").join("KingOfMonsters").join("SQLite").join("KOM.db"))?;

    let mut stmt = conn.prepare("SELECT * FROM monster_view")?;
    let monsters = stmt.query_map([], |row| {
        Ok(Monster {name: row.get(0)?, physical: row.get(1)?, race: row.get(2)?,
            condition: row.get(3)?, rank: row.get(4)?, age: row.get(5)?,
        })
    })?;

    for monster in monsters {
        println!("{:?}", monster?);
    }

/*
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
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
        println!("Found person {:?}", person?);
    }
*/
    Ok(())
}