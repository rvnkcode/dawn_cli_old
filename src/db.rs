use crate::{
    cli::{AddArgs, EditArgs},
    todo::Todo,
};
use rusqlite::{params_from_iter, Connection};
use std::path::PathBuf;

pub fn initialize_db(path: &PathBuf) {
    if !path.exists() {
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(include_str!("./sql/schema.sql"))
            .expect("Table creation failed");
        println!("...DB initialized");
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
        Ok(result) => println!("Created task {}", result),
        Err(e) => panic!("Failed to create a new To-Do: {:?}", e),
    };
}

pub fn get_todos(path: &PathBuf) -> Vec<Todo> {
    let conn = Connection::open(&path).unwrap();
    let mut stmt = conn
        .prepare("SELECT id, title FROM todo WHERE is_completed = 0 AND is_deleted = 0")
        .unwrap();

    stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed_at: None,
        })
    })
    .unwrap()
    .map(|r| r.unwrap())
    .collect::<Vec<Todo>>()
}

pub fn get_all_todos(path: &PathBuf) -> Vec<Todo> {
    let conn = Connection::open(&path).unwrap();
    let mut stmt = conn
        .prepare("SELECT id, title, completed_at FROM todo WHERE is_deleted = 0")
        .unwrap();

    stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed_at: row.get(2)?,
        })
    })
    .unwrap()
    .map(|r| r.unwrap())
    .collect::<Vec<Todo>>()
}

pub fn get_completed_todos(path: &PathBuf) -> Vec<Todo> {
    let conn = Connection::open(&path).unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT id, title, completed_at FROM todo WHERE is_completed = 1 AND is_deleted = 0",
        )
        .unwrap();

    stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed_at: row.get(2)?,
        })
    })
    .unwrap()
    .map(|r| r.unwrap())
    .collect::<Vec<Todo>>()
}

pub fn is_todo_exists(conn: &Connection, id: &u32) -> bool {
    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM todo WHERE id = (?1) AND is_deleted = 0)",
        [&id],
        |row| row.get(0),
    )
    .unwrap()
}

pub fn complete_todos(path: &PathBuf, ids: &Vec<u32>) {
    let mut result = ids.clone();
    let conn = Connection::open(&path).unwrap();

    result.retain(|id| is_todo_exists(&conn, &id));
    let count = result.len();
    if count > 0 {
        conn.execute(
            &format!(
                "UPDATE todo SET is_completed = 1 WHERE id IN ({}) AND is_completed = 0",
                repeat_vars(count)
            ),
            params_from_iter(&result),
        )
        .expect("Update faild");
        println!("Completed To-Do {:?}", &result);
    }
}

pub fn uncheck_todos(path: &PathBuf, ids: &Vec<u32>) {
    let mut result = ids.clone();
    let conn = Connection::open(&path).unwrap();

    result.retain(|id| is_todo_exists(&conn, &id));
    let count = result.len();
    if count > 0 {
        conn.execute(
            &format!(
                "UPDATE todo SET is_completed = 0 WHERE id IN ({}) AND is_completed = 1",
                repeat_vars(count)
            ),
            params_from_iter(&result),
        )
        .expect("Update faild");
        println!("Unfinished To-Do {:?}", &result);
    }
}

pub fn update_title(path: &PathBuf, todo: &EditArgs) {
    let conn = Connection::open(&path).unwrap();

    if is_todo_exists(&conn, &todo.id) {
        conn.execute(
            "UPDATE todo SET title = (?1) WHERE id = (?2)",
            (&todo.title, &todo.id),
        )
        .expect("Failed to update To-Do");

        println!("Updated To-Do {:?}", &todo.id);
    } else {
        println!("Check again your input: ID {:?}", &todo.id);
    }
}

pub fn restore_seeds(path: &PathBuf) {
    let conn = Connection::open(&path).unwrap();
    reset_db_before_seeding(&conn);
    seeding(&conn);
}

fn reset_db_before_seeding(conn: &Connection) {
    conn.execute("DELETE FROM todo", ()).ok();
}

fn seeding(conn: &Connection) {
    let count: u32 = conn
        .query_row("SELECT COUNT(*) FROM todo", [], |row| row.get(0))
        .unwrap();
    if count < 1 {
        conn.execute(include_str!("./sql/seed.sql"), ())
            .expect("Seeding failed");
        println!("...Seeding completed");
    }
}

pub fn reset_db(path: &PathBuf) {
    let conn = Connection::open(&path).unwrap();
    conn.execute("DELETE FROM todo", ()).ok();
    println!("...DB reset completed")
}

// Ref: https://docs.rs/rusqlite/latest/rusqlite/struct.ParamsFromIter.html#realistic-use-case
fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}
