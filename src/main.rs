#[macro_use]
extern crate prettytable;

use clap::Parser;

mod api;
mod args;
mod config;
mod models;

use crate::api::board::{get_board, get_boards};
use crate::api::issue::{get_issue, get_issues};

use crate::args::board::BoardCommands;
use crate::args::cli::{Cli, Commands};
use crate::args::issue::IssueCommands;
use crate::config::{config_file_path, create_config, Config};

//TODO change to use tabled instead of prettytable

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let path = config_file_path();
    if !path.exists() {
        create_config(path);
    }
    let mut config_data = Config::new();
    let client = reqwest::Client::new();
    match &cli.commands {
        Commands::Issue(params) => match &params.command {
            IssueCommands::List(params) => match get_issues(
                client,
                config_data.domain,
                config_data.auth_user.user,
                config_data.auth_user.jira_api_key,
                params,
            )
            .await
            {
                Ok(res) => println!("{}", res),
                Err(err) => println!("{err}"),
            },
            IssueCommands::Select(params) => match get_issue(
                client,
                config_data.domain,
                config_data.auth_user.user,
                config_data.auth_user.jira_api_key,
                &params.ticket,
            )
            .await
            {
                Ok(res) => println!("{res}"),
                Err(err) => println!("{err}"),
            },
            IssueCommands::Create => {
                println!("Create")
            }
        },
        Commands::Boards(params) => match &params.command {
            BoardCommands::List(params) => {
                match get_boards(
                    client,
                    config_data.domain,
                    config_data.auth_user.user,
                    config_data.auth_user.jira_api_key,
                    params.page,
                )
                .await
                {
                    Ok(res) => println!("{}  \nPage: {} / {}", res, params.page, res.total / 50),
                    Err(err) => println!("{err}"),
                }
            }
            BoardCommands::Select(params) => {
                match get_board(
                    client,
                    &config_data.domain,
                    &config_data.auth_user.user,
                    &config_data.auth_user.jira_api_key,
                    params.id,
                )
                .await
                {
                    Ok(res) => {
                        println!("{res}");
                        config_data.update_field(params.id);
                    }
                    Err(err) => println!("{err}"),
                }
            }
        },
    }
}
