use prettytable::{format, row, Table};

use crate::todo::Todo;

pub fn print_list(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row!["ID", "Title"]);

    for todo in list {
        table.add_row(row![todo.id, todo.title,]);
    }

    table.printstd();
}

pub fn print_list_with_completion_date(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row!["ID", "Title", "Completion Date"]);

    for todo in list {
        table.add_row(row![
            todo.id,
            todo.title,
            todo.completed_at.clone().get_or_insert("-".to_string())
        ]);
    }

    table.printstd();
}
