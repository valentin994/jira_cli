use crate::args::board::BoardArgs;
use crate::args::issue::IssueArgs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // /// Sprint Operations
    // SprintEnum(SprintCommand),
    // /// List all projects
    // List(ListCommand),
    /// List issues
    Issue(IssueArgs),

    /// Jira board operations, list shows all available boards to you.
    Boards(BoardArgs),
}
