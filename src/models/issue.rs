use chrono::DateTime;
use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::fmt;

use prettytable::Table;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Issues {
    expand: String,
    start_at: Option<u16>,
    max_results: Option<u8>,
    total: u32,
    issues: Vec<Issue>,
}

impl fmt::Display for Issues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["ID", "Summary", "Status", "Assignee", "Creator", "Priority"]);
        write!(f, "\nTotal issues: {}\n", self.total)?;
        for issue in &self.issues {
            table.add_row(row![
                issue.key,
                issue.fields.summary,
                issue.fields.status,
                issue.fields.assignee.display_name,
                issue.fields.creator.display_name,
                issue.fields.priority.name
            ]);
            // write!(f, "\n{issue}\n")?;
        }
        table.printstd();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    key: String,
    fields: Fields,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fields {
    parent: Option<Parent>,
    priority: Priority,
    assignee: Assignee,
    status: Status,
    creator: Creator,
    reporter: Reporter,
    issuetype: Issuetype,
    created: String,
    description: Option<String>,
    summary: String, //    sprint: Sprint
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sprint {
    id: u16,
    state: String,
    name: String,
    start_date: String,
    end_date: String,
    created_date: String,
    origin_board_id: u16,
    goal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issuetype {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Reporter {
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    name: String,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Assignee {
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parent {
    key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Priority {
    name: String,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
