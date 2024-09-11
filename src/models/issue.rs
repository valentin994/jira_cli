use chrono::DateTime;
use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::fmt;

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
        write!(f, "Total issues this sprint: {}", self.total)?;
        for issue in &self.issues {
            write!(f, "\n{issue}\n")?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    key: String,
    fields: Fields,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{} {}", self.key.clone().bold().green(), self.fields)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fields {
    parent: Option<Parent>,
    priority: Option<Priority>,
    assignee: Option<Assignee>,
    status: Status,
    creator: Creator,
    reporter: Reporter,
    issuetype: Issuetype,
    created: String,
    description: Option<String>,
    summary: String, //    sprint: Sprint
}

impl fmt::Display for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let priority = match &self.priority {
            Some(value) => value.clone(),
            None => Priority {
                name: "No priority set".to_string(),
            },
        };
        let description = match &self.description {
            Some(value) => value.clone(),
            None => "No description".to_string(),
        };
        let datetime = DateTime::parse_from_str(&self.created, "%Y-%m-%dT%H:%M:%S%.3f%z")
            .expect("Failed to parse date");
        write!(
            f,
            "{} \n\nDescription:\n------------ \n{}\n \n|- Priority: {} \n|- Status: {} \n|- Created: {}",
            self.summary.clone().bold().red(),
            description,
            priority,
            self.status,
            datetime.format("%d-%m-%Y")
        )?;
        Ok(())
    }
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
    display_name: String,
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
