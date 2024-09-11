use clap::{Args, Parser, Subcommand};

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

#[derive(Args, Debug)]
pub struct SprintCommand {}

#[derive(Args, Debug)]
pub struct ListCommand {
    /// Pagination
    #[arg(default_value_t = 0)]
    pub page: u8,
}
