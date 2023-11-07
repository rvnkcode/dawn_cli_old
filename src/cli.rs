use clap::{Args, Parser, Subcommand};
use std::io::{stdin, stdout, Result, Write};

#[derive(Parser)]
#[command(bin_name = "dawn", author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new To-Do
    Add(AddArgs),
    /// List To-Dos
    Ls(ListArgs),
    /// List all To-Dos
    All(ListArgs),
    /// List completed To-Dos
    Completed(ListArgs),
    /// List deleted To-Dos
    Trash(ListArgs),
    /// Display details of a To-Do
    Show {
        id: u32
    },
    /// Complete To-Dos
    Done(IdsArgs),
    /// Revert To-Dos to an unfinished state
    Undone(IdsArgs),
    /// Modify a To-Do
    Modify(EditArgs),
    /// Delete To-Dos
    Delete(IdsArgs),
    /// Permanently delete trashed To-Dos
    Clean,
    /// Reset DB
    Reset,
    /// Reset DB and restore seed. Only for development!
    #[command(hide = true)]
    Seed,
}

#[derive(Args)]
pub struct AddArgs {
    pub title: String,
    #[arg(short, long, default_value_t = false)]
    pub check: bool,
    #[arg(short, long)]
    pub note: Option<String>,
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long)]
    pub note: bool,
}

#[derive(Args, Debug)]
pub struct IdsArgs {
    #[arg(required = true)]
    pub ids: Vec<u32>,
}

#[derive(Args)]
pub struct EditArgs {
    pub id: u32,
    pub title: String,
}

#[derive(PartialEq)]
pub enum PromptResult {
    Yes,
    No,
}

pub fn propmt_user(prompt: &str) -> Result<Option<PromptResult>> {
    let mut input = String::new();
    loop {
        print!("{prompt} ");
        stdout().flush()?;
        input.clear();
        stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(Some(PromptResult::Yes)),
            "n" | "no" => return Ok(Some(PromptResult::No)),
            "" => return Ok(None),
            _ => {
                println!("Error: Unrecognized option '{}'", input.trim());
                println!("Note: Press Ctrl+C to exit");
            }
        }
    }
}
