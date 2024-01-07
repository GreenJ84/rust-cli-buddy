use rusqlite::Connection;

pub fn database_establishment(){
    match Connection::open("./buddy_db.db3"){
        Ok(conn) => {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS passwords (
                    id INTEGER PRIMARY KEY,
                    site TEXT NOT NULL,
                    username TEXT NOT NULL,
                    email TEXT NOT NULL,
                    password TEXT NOT NULL
                )",
                [],
            ).unwrap();

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

            if let Err( (conn, err) ) = conn.close() {
                eprint!("Failed to close the database connection: {:?}\n", err);
                conn.close().unwrap();
            }
        }
        Err(e) => {
            println!("Error connecting to the Buddy database: {}", e);
        }
    }
}
