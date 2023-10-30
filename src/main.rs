use rusqlite::Connection;
use std::fs::create_dir;

fn main() {
    let path = dirs::home_dir().unwrap().join(".dawn");
    /*
    println!("{:}", path.display()); // Debug
    */

    if path.exists() {
        println!("...Directory checked");
    } else {
        create_dir(&path).expect("Error");
        println!("...Directory created");
    }

    // Create sqlite .db file and todo table
    let path = path.join("todo.db");
    let conn = Connection::open(&path).expect("Error");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
                id      INTEGER PRIMARY KEY,
                name    TEXT NOT NULL
            )",
        (),
    ).expect("Error");
}
