use serde::{Deserialize, Serialize};
use std::fmt;

use prettytable::Table;

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
        let mut table = Table::new();
        table.add_row(row![FG -> "ID", FG -> "NAME"]);
        write!(f, "\nTotal boards: {}\n", self.total)?;
        for board in &self.values {
            table.add_row(row![board.id, board.name]);
        }
        table.printstd();
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
