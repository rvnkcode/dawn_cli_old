use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(bin_name = "dawn")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
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

#[derive(ValueEnum, Clone, Debug)]
pub enum ListFilters {
    All,
    End,
}
