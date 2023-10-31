use rusqlite::Connection;
use std::{fs::create_dir, path::PathBuf};

#[warn(dead_code)]
struct Todo {
    id: u32,
    title: String,
    is_completed: bool,
    // TODO: Handling date and time with Rust
}

fn main() {
    let path = define_directory();
    // println!("{:}", path.display()); // Debug
    check_directory(&path);

    let path = path.join("todo.db");
    check_db(&path);
}

fn define_directory() -> PathBuf {
    let path = match dirs::home_dir() {
        Some(p) => p,
        None => PathBuf::new(),
    };

    path.join(".dawn")
}

fn check_directory(path: &PathBuf) {
    if path.exists() {
        println!("...Directory checked");
    } else {
        create_dir(&path).expect("Directory creation failed");
        println!("...Directory created");
    }
}

fn check_db(path: &PathBuf) {
    if path.exists() {
        println!("...DB checked");
    } else {
        let conn = Connection::open(&path).expect("Connection open failed");
        initialize_db(&conn);
        seeding(&conn);
    }
}

fn initialize_db(conn: &Connection) {
    conn.execute(include_str!("./sql/schema.sql"), ())
        .expect("Error");
    println!("...DB initialized");
}

fn seeding(conn: &Connection) {
    let count: u32 = conn
        .query_row("SELECT COUNT(*) FROM todo", [], |row| row.get(0))
        .expect("To-do count query failed");
    if count < 1 {
        conn.execute(include_str!("./sql/seed.sql"), ())
            .expect("Seeding failed");
        println!("...Seeding completed");
    }
}
