use std::env;
use std::path::Path;
use std::collections::HashMap;
use rusqlite::{params, Connection, Result};

struct StatusRow {
    monster_id: i32,
    status_id: i32,
    name: String,
    value: i32,
}

#[derive(Debug, Clone)]
struct Status {
    status_id: i32,
    name: String,
    value: i32,
}

#[derive(Debug)]
struct Monster {
    id: i32,
    name: String,
    physical: i32,
    race: String,
    condition: String,
    rank: String,
    age: i32,
    status: Vec<Status>,
}

fn main() -> Result<()> {


    let conn = Connection::open(Path::new(&env::var("PROGRAMDATA").unwrap()).join("TakmaruApp").join("KingOfMonsters").join("SQLite").join("KOM.db"))?;

    let mut monster_status_map: HashMap<i32, Vec<Status>> = HashMap::new();
    {
        let mut stmt = conn.prepare("SELECT * FROM monster_status_view")?;
        let monster_status_view = stmt.query_map([], |row| {
            Ok(StatusRow {monster_id: row.get(0)?, status_id: row.get(1)?, name: row.get(2)?, value: row.get(3)?})
        })?;
        for status_row in monster_status_view {
            let st = status_row?;
            monster_status_map
                .entry(st.monster_id)
                .or_insert_with(Vec::new)
                .push(Status{ status_id: st.status_id, name: st.name, value: st.value });
        }
    }
    let monster_status_map = monster_status_map; // イミュータブルにする

    let mut stmt = conn.prepare("SELECT * FROM monster_view")?;
    let monsters = stmt.query_map([], |row| {
        let id = row.get(0)?;
        Ok(Monster {id, name: row.get(1)?, physical: row.get(2)?, race: row.get(3)?,
            condition: row.get(4)?, rank: row.get(5)?, age: row.get(6)?, status: monster_status_map.get(&id).cloned().unwrap_or_default(),
        })
    })?;

    // モンスター一覧を表示する
    for monster in monsters {
        println!("{:?}", monster?);
    }

    // 育成するモンスターを選択する
    // -> 1

    // モンスターのステータスを取得する

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