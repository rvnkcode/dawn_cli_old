use crate::cli::ListFilters;
use clap::Parser;
use cli::{propmt_user, Cli, Commands, PromptResult};
use config::{check_directory, define_directory};
use db::{
    complete_todos, create_todo, get_all_todos, get_completed_todos, get_todos, initialize_db,
    reset_db, restore_seeds, uncheck_todos, update_title,
};
use table::{print_list, print_list_with_completion_date};

mod cli;
mod config;
mod db;
mod table;
mod todo;

fn main() {
    let path = define_directory();
    check_directory(&path);

    let path = path.join("todo.db");
    initialize_db(&path);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(todo) => create_todo(&todo, &path),
        Commands::Ls(list_args) => match &list_args.filter {
            Some(ListFilters::All) => print_list_with_completion_date(&get_all_todos(&path)),
            Some(ListFilters::End) => print_list_with_completion_date(&get_completed_todos(&path)),
            None => print_list(&get_todos(&path)),
        },
        Commands::Done(check_args) => complete_todos(&path, &check_args.ids),
        Commands::Undone(check_args) => uncheck_todos(&path, &check_args.ids),
        Commands::Edit(todo) => update_title(&path, &todo),
        Commands::Reset => {
            if propmt_user("WARNING: this operation cannot be undone and every data will be permanently deleted: [y/N]").unwrap() == Some(PromptResult::Yes)
            {
                reset_db(&path);
            }
        }
        Commands::Seed => restore_seeds(&path),
    }
}
