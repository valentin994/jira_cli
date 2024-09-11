use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    name: String,
    id: String,
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    total: u8,
    values: Vec<Project>,
}
