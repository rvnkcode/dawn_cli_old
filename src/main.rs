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
    let path = define_path();
    // println!("{:}", path.display()); // Debug
    check_directory(&path);

    let path = path.join("todo.db");
    initialize_db(&path);
}

fn define_path() -> PathBuf {
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
        create_dir(&path).expect("Error");
        println!("...Directory created");
    }
}

fn initialize_db(path: &PathBuf) {
    let conn = Connection::open(&path).expect("Error");
    conn.execute(include_str!("./sql/schema.sql"), ())
        .expect("Error");
    println!("...DB initialized");
}
