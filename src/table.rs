use prettytable::{format, row, Table};

use crate::todo::Todo;

pub fn print_list(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row![u => "ID", "Title"]);

    for todo in list {
        table.add_row(row![&todo.id, &todo.title]);
    }

    table.printstd();
}

pub fn print_list_with_note(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row![u => "ID", "Title", "Note"]);

    for todo in list {
        table.add_row(row![
            &todo.id,
            &todo.title,
            if todo.note.is_some() {
                todo.note.clone().unwrap()
            } else {
                "".to_string()
            }
        ]);
    }

    table.printstd();
}

pub fn print_list_with_details(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row![u=> "ID", "St", "Title", "Note", "Completion Date"]);

    for todo in list {
        table.add_row(row![
            &todo.id,
            if todo.is_completed {
                "C".to_string()
            } else {
                "P".to_string()
            },
            &todo.title,
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

pub fn print_detail_of_todo(todo: &Todo) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    table.add_row(row!["ID", &todo.id]);
    table.add_row(row!["Title", &todo.title]);
    table.add_row(row![
        "Note",
        if todo.note.is_some() {
            todo.note.clone().unwrap()
        } else {
            "".to_string()
        }
    ]);
    table.add_row(row![
        "Creation Date",
        if todo.created_at.is_some() {
            todo.created_at
                .unwrap()
                .format("%Y-%m-%d %H:%M")
                .to_string()
        } else {
            "".to_string()
        }
    ]);
    table.add_row(row![
        "Completion Date",
        if todo.completed_at.is_some() {
            todo.completed_at
                .unwrap()
                .format("%Y-%m-%d %H:%M")
                .to_string()
        } else {
            "".to_string()
        }
    ]);

    table.printstd();
}
