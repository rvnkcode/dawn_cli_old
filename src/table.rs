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

pub fn print_list_with_note(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row!["ID", "Title", "Note"]);

    for todo in list {
        table.add_row(row![
            todo.id,
            todo.title,
            if todo.note.is_some() {
                todo.note.clone().unwrap()
            } else {
                "".to_string()
            }
        ]);
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
            if todo.completed_at.is_some() {
                todo.completed_at
                    .unwrap()
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            } else {
                "".to_string()
            }
        ]);
    }

    table.printstd();
}

pub fn print_list_with_completion_date_note(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row!["ID", "Title", "Note", "Completion Date"]);

    for todo in list {
        table.add_row(row![
            todo.id,
            todo.title,
            if todo.note.is_some() {
                todo.note.clone().unwrap()
            } else {
                "".to_string()
            },
            if todo.completed_at.is_some() {
                todo.completed_at
                    .unwrap()
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            } else {
                "".to_string()
            }
        ]);
    }

    table.printstd();
}
