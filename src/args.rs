use clap::{Args, Parser, Subcommand};
use crate::config::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Sprint Operations
    SprintEnum(SprintCommand),
    /// List all projects
    List(ListCommand),

    /// List issues
    Issue(IssueArgs),

    /// Jira board operations, list shows all available boards to you.
    Boards(BoardArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct IssueArgs {
    #[command(subcommand)]
    pub command: Option<IssueCommands>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum IssueCommands {
    List,
    Create,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BoardArgs {
    #[command(subcommand)]
    pub command: BoardCommands,
}

#[derive(Debug, Subcommand)]
pub enum BoardCommands {
    /// List boards
    List(BoardListArgs),

    /// Select a board to do operations on that board
    Select(BoardSelectArgs),
}

#[derive(Debug, Args)]
pub struct BoardListArgs {
    #[arg(default_value_t = 1)]
    pub page: u8,
}

#[derive(Debug, Args)]
pub struct BoardSelectArgs {
    #[arg()]
    pub id: u16,
}

#[derive(Args, Debug)]
pub struct SprintCommand {}

#[derive(Args, Debug)]
pub struct ListCommand {
    /// Pagination
    #[arg(default_value_t = 0)]
    pub page: u8,
}
