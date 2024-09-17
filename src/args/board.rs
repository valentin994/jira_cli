use clap::{Args, Subcommand};

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
