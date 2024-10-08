use std::{
    fs,
    io::{self, BufReader},
    path::Path,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub auth_user: AuthUser,
    pub domain: String,
    pub active_board: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub user: String,
    pub jira_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        let file = Config::load_config_file();
        let reader = BufReader::new(file.unwrap());
        let config_data: Config = serde_json::from_reader(reader).unwrap();
        config_data
    }

    fn load_config_file() -> Result<fs::File, io::Error> {
        let path = config_file_path();
        let file = std::fs::File::options().read(true).open(path);
        file
    }

    pub fn update_field(&mut self, board_id: u16) -> &mut Self {
        let path = config_file_path();
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        self.active_board = Some(board_id);
        serde_json::to_writer(file, &self).unwrap();
        self
    }
}

pub fn config_file_path() -> std::path::PathBuf {
    let project_directory = ProjectDirs::from("", "", "jira_cli");
    let path = project_directory
        .clone()
        .unwrap()
        .config_dir()
        .join(Path::new("config.json"));
    path
}

pub fn create_config(path: std::path::PathBuf) {
    println!("Jira user:");
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to parse user");
    input_user.pop();
    println!("API KEY:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read API KEY");
    input.pop();
    println!("Domain:");
    let mut input_domain = String::new();
    io::stdin()
        .read_line(&mut input_domain)
        .expect("Failed to parse domain");
    input_domain.pop();
    let auth_user = AuthUser {
        user: input_user,
        jira_api_key: input,
    };
    let _ = fs::create_dir_all(ProjectDirs::from("", "", "jira_cli").unwrap().config_dir());
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    let config = Config {
        auth_user,
        domain: input_domain,
        active_board: None,
    };
    serde_json::to_writer(file, &config).unwrap();
}
