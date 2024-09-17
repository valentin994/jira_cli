use crate::args::issue::IssueListArgs;
use crate::models::issue::{Issue, Issues};

pub async fn get_issues(
    client: reqwest::Client,
    domain: String,
    user: String,
    api_key: String,
    params: &IssueListArgs,
) -> Result<Issues, Box<dyn std::error::Error>> {
    let mut jql = String::from("&jql=");
    if let Some(status) = &params.status {
        jql.push_str(&format!("status='{}' ", status));
    }
    if let Some(assignee) = &params.assignee {
        if !jql.trim_end().is_empty() {
            jql.push_str("AND "); // Add a separator if there's already a previous condition
        }
        jql.push_str(&format!("assignee='{}'", assignee));
    }
    let url = format!("https://{domain}/rest/agile/1.0/board/333/sprint/7281/issue?fields=summary,priority,assignee,status, customfield_10004, creator,reporter,created,description,issuetype{jql}");
    let body = client
        .get(url)
        .basic_auth(user, Some(api_key))
        .send()
        .await?
        .text()
        .await?;
    let v: Issues = serde_json::from_str(&body)?;
    Ok(v)
}

pub async fn get_issue(
    client: reqwest::Client,
    domain: String,
    user: String,
    api_key: String,
    ticket: &String,
) -> Result<Issue, Box<dyn std::error::Error>> {
    let url = format!("https://{domain}/rest/api/2/issue/{ticket}");
    let body = client
        .get(url)
        .basic_auth(user, Some(api_key))
        .send()
        .await?
        .text()
        .await?;
    let v: Issue = serde_json::from_str(&body)?;
    Ok(v)
}
