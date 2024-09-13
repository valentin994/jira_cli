use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Boards {
    max_results: Option<u16>,
    start_at: Option<u16>,
    pub total: u16,
    is_last: bool,
    values: Vec<Board>,
}

impl fmt::Display for Boards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Total boards: {}", self.total)?;
        for board in &self.values {
            write!(f, "\n{board}")?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    id: u16,
    name: String,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}, Name: {}", self.id, self.name)
    }
}
