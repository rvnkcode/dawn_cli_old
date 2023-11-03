use clap::Parser;
use cli::{Cli, Commands};
use config::{check_directory, define_directory};
use db::{
    check_db, complete_todos, create_todo, get_all_todos, get_completed_todos, get_todos,
    restore_seeds,
};
use table::print_list;
use todo::Todo;

use crate::cli::ListFilters;

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
        Commands::Add(todo) => create_todo(&todo, &path),
        Commands::Ls(list_args) => {
            let list: Vec<Todo> = match &list_args.filter {
                Some(ListFilters::All) => get_all_todos(&path),
                Some(ListFilters::End) => get_completed_todos(&path),
                None => get_todos(&path),
            };
            print_list(&list);
        }
        Commands::Check(check_args) => complete_todos(&path, &check_args.ids),
        Commands::Seed => restore_seeds(&path),
    }
}
