use clap::Parser;
use cli::{Cli, Commands};
use config::{check_directory, define_directory};
use db::{check_db, create_todo, get_todos};
use table::print_list;

mod cli;
mod config;
mod db;
mod table;
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
            print_list(&list);
        }
    }
}
