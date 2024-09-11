use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprints {
    total: u8,
    values: Vec<Sprint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprint {
    id: i32,
    state: String,
    name: String,
}
