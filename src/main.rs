use db::{check_db, check_directory, define_directory};

mod db;

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
