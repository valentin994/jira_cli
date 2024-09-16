use clap::{Args, Parser, Subcommand, ValueEnum};

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
    pub command: IssueCommands,
}

#[derive(Debug, Subcommand)]
pub enum IssueCommands {
    List(IssueListArgs),
    Create,
}

#[derive(Debug, Clone, Args)]
pub struct IssueListArgs {
    #[arg(short, long)]
    pub status: Option<String>,
    #[arg(short, long)]
    pub assignee: Option<String>,
    #[arg(short, long)]
    pub creator: Option<String>,
    #[arg(short, long)]
    pub priority: Option<PriorityArg>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PriorityArg {
    NeedsPriority,
    Low,
    Medium,
    High,
    Critical
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
