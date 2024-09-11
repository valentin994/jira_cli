use std::{
    fs,
    io::{self, BufReader},
    path::Path,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
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
    println!("API KEY:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read API KEY");
    input.pop();
    let _ = fs::create_dir_all(ProjectDirs::from("", "", "jira_cli").unwrap().config_dir());
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    let config = Config {
        jira_api_key: input,
    };
    serde_json::to_writer(file, &config).unwrap();
}
