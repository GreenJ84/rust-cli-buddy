pub mod models;
pub mod controllers;

use std::collections::HashMap;
use rusqlite::{Connection, Result, Error};

pub fn database_establishment() -> Result<(), Error>{
    // Track if needed tables exists in app
    let mut table_check = HashMap::from([
        ("user", false),
        ("tasks", false),
        ("passwords", false),
    ]);
    // Open SQLite DB
    match Connection::open("./buddy_db.db3"){
        Ok(conn) => {
            // Get all DB tables
            let mut stmt = conn.prepare(
                "SELECT name FROM sqlite_master WHERE type = 'table';",
            )?;

            // Iterate present tables to ensure app db is filled
            let tables = stmt.query_map([], |row| row.get::<_, String>(0))?;
            for entry in tables {
                if let Ok(table) = entry {
                    // If they are ours from Buddy adjust the tracker
                    if table_check.contains_key(&table[..]) {
                        *table_check.get_mut(&table[..]).unwrap() = true;
                    }
                } 
            }

            if !(*table_check.get("user").unwrap()) {
                conn.execute(
                    "CREATE TABLE IF NOT EXISTS user (
                        id INTEGER PRIMARY KEY,
                        name TEXT,
                        username TEXT NOT NULL,
                        email TEXT,
                        secret TEXT
                    )",
                    [],
                ).unwrap();
            }

            if !(*table_check.get("passwords").unwrap()) {
                conn.execute(
                    "CREATE TABLE IF NOT EXISTS passwords (
                        id INTEGER PRIMARY KEY,
                        site TEXT,
                        username TEXT NOT NULL,
                        email TEXT NOT NULL,
                        password TEXT NOT NULL
                    )",
                    [],
                ).unwrap();
            }

            if !(*table_check.get("tasks").unwrap()) {
                conn.execute(
                    "CREATE TABLE IF NOT EXISTS tasks (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        title TEXT NOT NULL,
                        description TEXT NOT NULL,
                        due_date TEXT,
                        priority INTEGER NOT NULL,
                        status TEXT NOT NULL,
                        created_at TEXT NOT NULL,
                        updated_at TEXT NOT NULL,
                        completed_at TEXT
                    )",
                    []
                ).unwrap();
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}