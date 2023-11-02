use clap::Parser;
use cli::{Cli, Commands};
use config::{check_directory, define_directory};
use db::{check_db, create_todo};

mod cli;
mod config;
mod db;

fn main() {
    let path = define_directory();
    // println!("{:}", path.display()); // Debug
    check_directory(&path);

    let path = path.join("todo.db");
    check_db(&path);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(todo) => {
            create_todo(&todo, &path);
        }
    }
}
