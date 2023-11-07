use clap::Parser;
use cli::{propmt_user, Cli, Commands, PromptResult};
use config::{check_directory, define_directory};
use db::{
    complete_todos, create_todo, delete_todos, empty_trash, get_all_todos, get_completed_todos,
    get_deleted_todos, get_detail_of_todo, get_todos, initialize_db, reset_db, restore_seeds,
    uncheck_todos, update_title,
};
use table::{
    print_detail_of_todo, print_list, print_list_with_completion_date,
    print_list_with_completion_date_note, print_list_with_note,
};

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
        // Create
        Commands::Add(todo) => create_todo(&todo, &path),
        // Read
        Commands::Ls(args) => {
            let list = get_todos(&path);
            if args.note {
                print_list_with_note(&list);
            } else {
                print_list(&list);
            }
        },
        Commands::All(args) => {
            let list = get_all_todos(&path);
            if args.note {
                print_list_with_completion_date_note(&list);
            } else {
                print_list_with_completion_date(&list);
            }
        },
        Commands::Completed(args) => {
            let list = get_completed_todos(&path);
            if args.note {
                print_list_with_completion_date_note(&list);
            } else {
                print_list_with_completion_date(&list);
            }
        },
        Commands::Trash(args) => {
            let list = get_deleted_todos(&path);
            if args.note {
                print_list_with_completion_date_note(&list);
            } else {
                print_list_with_completion_date(&list);
            }
        },
        Commands::Show { id } => {
            print_detail_of_todo(&get_detail_of_todo(&path, &id));
        }
        // Update
        Commands::Done(ids_args) => complete_todos(&path, &ids_args.ids),
        Commands::Undone(ids_args) => uncheck_todos(&path, &ids_args.ids),
        Commands::Modify(todo) => update_title(&path, &todo),
        // Delete
        Commands::Delete(ids_args) => delete_todos(&path, &ids_args.ids),
        Commands::Clean => empty_trash(&path),
        Commands::Reset => {
            if propmt_user("WARNING: this operation cannot be undone and every data will be permanently deleted: [y/N]").unwrap() == Some(PromptResult::Yes)
            {
                reset_db(&path);
            }
        }
        // Development
        Commands::Seed => restore_seeds(&path),
    }
}
