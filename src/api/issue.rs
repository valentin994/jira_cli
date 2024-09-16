use crate::models::issue::Issues;
use crate::args::IssueListArgs;

pub async fn get_issues(
    client: reqwest::Client,
    domain: String,
    user: String,
    api_key: String,
    params: &IssueListArgs 
) -> Result<Issues, Box<dyn std::error::Error>> {
    let mut jql = String::from("&jql=");
    let _status = if let Some(ref x) = params.status {
        jql.push_str(&format!("status='{}'", x));
        x.clone()
    } else {
        String::new()
    };
    let _assignee = if let Some(ref x) = params.assignee {
        jql.push_str(&format!("assignee='{}'", x));
        x.clone()
    } else {
        String::new()
    };
    let url = format!("https://{domain}/rest/agile/1.0/board/333/sprint/7281/issue?fields=summary,priority,assignee,status,creator,reporter,created,description,issuetype{jql}");
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
