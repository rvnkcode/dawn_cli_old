use prettytable::{Table, row, format};

use crate::todo::Todo;

pub fn print_list(list: &Vec<Todo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.set_titles(row!["ID", "Title"]);

    for todo in list {
        table.add_row(row![todo.id, todo.title]);
    }

    table.printstd();
}
