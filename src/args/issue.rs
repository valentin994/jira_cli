use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct IssueArgs {
    #[command(subcommand)]
    pub command: IssueCommands,
}

#[derive(Debug, Subcommand)]
pub enum IssueCommands {
    /// Select issue specifying the ticket number f.e. DATA-13000
    Select(SelectArgs),
    /// List issues from the selected sprint with specified filters
    List(IssueListArgs),
    Create,
}

#[derive(Debug, Clone, Args)]
pub struct SelectArgs {
    #[arg(short, long)]
    pub ticket: String,
}

#[derive(Debug, Clone, Args)]
pub struct IssueListArgs {
    /// Status of the task, possible values "Open", "In Progress", "Closed", "Needs more info",
    /// "Ready for Dev"
    #[arg(short, long)]
    pub status: Option<String>,
    /// Assignee, use full name, f.e. "John Doe"
    #[arg(short, long)]
    pub assignee: Option<String>,
    /// Creator, use full name, f.e. "John Doe"
    #[arg(short, long)]
    pub creator: Option<String>,
    /// Priority of the taks, possible values "Low", "Medium", "High", "Critical", "Unassigned",
    /// "Needs Priority"
    #[arg(short, long)]
    pub priority: Option<String>,
}
