use clap::Parser;
use cli::{Cli, Commands};
use config::{check_directory, define_directory};
use db::{check_db, create_todo, get_todos};

mod cli;
mod config;
mod db;
mod todo;

fn main() {
    let path = define_directory();
    check_directory(&path);

    let path = path.join("todo.db");
    check_db(&path);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(todo) => {
            create_todo(&todo, &path);
        }
        Commands::Ls => {
            let list = get_todos(&path);
            for todo in list {
                println!("{0} {1}", todo.id, todo.title);
            }
        }
    }
}
