use clap::Parser;

mod api;
mod args;
mod config;
mod models;

use crate::api::board::{get_board, get_boards};
use crate::api::issue::get_issues;
use crate::api::projects::get_projects;
use crate::api::sprint::get_current_sprint;

use crate::args::{BoardCommands, Cli, Commands, IssueCommands};
use crate::config::{config_file_path, create_config, Config};

// TODO error handling
// TODO different displays for issues list

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
        Commands::List(params) => {
            match get_projects(client, config_data.domain, config_data.jira_api_key, params.page).await {
                Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
                Err(err) => println!("{err}"),
            }
        }
        Commands::SprintEnum(_params) => {
            match get_current_sprint(client, config_data.jira_api_key).await {
                Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
                Err(err) => println!("{err}"),
            }
        }
        Commands::Issue(params) => {
            let issue_command = params.command.clone().unwrap_or(IssueCommands::List);
            match issue_command {
                IssueCommands::List => match get_issues(client, config_data.domain, config_data.jira_api_key).await {
                    Ok(res) => println!("{}", res),
                    Err(err) => println!("{err}"),
                },
                IssueCommands::Create => {
                    println!("Create")
                }
            }
        }
        Commands::Boards(params) => match &params.command {
            BoardCommands::List(params) => {
                match get_boards(client, config_data.domain, config_data.jira_api_key, params.page).await {
                    Ok(res) => println!("{}  \nPage: {} / {}", res, params.page, res.total / 50),
                    Err(err) => println!("{err}"),
                }
            }
            BoardCommands::Select(params) => {
                match get_board(client, &config_data.domain, &config_data.jira_api_key, params.id).await {
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
