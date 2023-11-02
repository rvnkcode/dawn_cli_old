use rusqlite::Connection;
use std::{fs::create_dir, path::PathBuf};

use crate::AddArgs;

pub fn check_db(path: &PathBuf) {
    if !path.exists() {
        let conn = Connection::open(&path).expect("Connection open failed");
        initialize_db(&conn);
        seeding(&conn);
    }
}

pub fn initialize_db(conn: &Connection) {
    conn.execute_batch(include_str!("./sql/schema.sql"))
        .expect("Table creation failed");
    println!("...DB initialized");
}

pub fn seeding(conn: &Connection) {
    let count: u32 = conn
        .query_row("SELECT COUNT(*) FROM todo", [], |row| row.get(0))
        .expect("To-do count query failed");
    if count < 1 {
        conn.execute(include_str!("./sql/seed.sql"), ())
            .expect("Seeding failed");
        println!("...Seeding completed");
    }
}

pub fn create_todo(todo: &AddArgs, path: &PathBuf) {
    let conn = Connection::open(path).expect("Connection open failed");
    conn.execute(
        "INSERT INTO todo (title, is_completed) VALUES (?1, ?2)",
        (&todo.title, &todo.check),
    )
    .expect("Failed to create To-Do");
}
