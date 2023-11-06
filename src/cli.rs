use clap::{Args, Parser, Subcommand, ValueEnum};
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
    /// Complete To-Dos
    Done(CheckArgs),
    /// Revert To-Dos to an unfinished state
    Undone(CheckArgs),
    /// Modify a To-Do
    Edit(EditArgs),
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
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(value_enum, short)]
    pub filter: Option<ListFilters>,
}

#[derive(Args, Debug)]
pub struct CheckArgs {
    pub ids: Vec<u32>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ListFilters {
    All,
    End,
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
