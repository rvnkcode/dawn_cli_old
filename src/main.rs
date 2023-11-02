use clap::{Args, Parser, Subcommand};
use db::{check_db, check_directory, define_directory};

use crate::db::create_todo;

mod db;
mod todo;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new To-Do
    Add(AddArgs),
}

#[derive(Args)]
pub struct AddArgs {
    title: String,
    #[arg(short, long, default_value_t = false)]
    check: bool,
}

fn main() {
    let path = define_directory();
    // println!("{:}", path.display()); // Debug
    check_directory(&path);

    let path = path.join("todo.db");
    check_db(&path);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(todo) => {
            // TODO: add check property
            create_todo(&todo.title, &path);
        }
    }
}
