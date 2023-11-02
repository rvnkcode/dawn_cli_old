use crate::cli::AddArgs;
use rusqlite::Connection;
use std::path::PathBuf;

pub fn check_db(path: &PathBuf) {
    if !path.exists() {
        let conn = Connection::open(&path).unwrap();
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
        .unwrap();
    if count < 1 {
        conn.execute(include_str!("./sql/seed.sql"), ())
            .expect("Seeding failed");
        println!("...Seeding completed");
    }
}

pub fn create_todo(todo: &AddArgs, path: &PathBuf) {
    let conn = Connection::open(&path).unwrap();
    conn.execute(
        "INSERT INTO todo (title, is_completed) VALUES (?1, ?2)",
        (&todo.title, &todo.check),
    )
    .expect("Failed to create To-Do");

    match conn.query_row(
        "SELECT id FROM todo WHERE title=(?1) ORDER BY updated_at DESC LIMIT 1",
        [&todo.title],
        |row| row.get::<_, u32>(0),
    ) {
        Ok(result) => println!("Created task {:?}", result),
        Err(e) => panic!("Failed to create a new To-Do: {:?}", e),
    };
}
