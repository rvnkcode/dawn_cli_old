use clap::{Args, Parser, Subcommand, ValueEnum};

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
    pub ids: Vec<u32>
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ListFilters {
    All,
    End,
}
