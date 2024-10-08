use crate::models::sprint::Sprints;

pub async fn get_current_sprint(
    client: reqwest::Client,
    domain: String,
    user: String,
    api_key: String,
) -> Result<Sprints, Box<dyn std::error::Error>> {
    let url = format!("https://{domain}/rest/agile/1.0/board/336/sprint?state=active");
    let body = client
        .get(url)
        .basic_auth(user, Some(api_key))
        .send()
        .await?
        .text()
        .await?;
    let v: Sprints = serde_json::from_str(&body)?;
    Ok(v)
}
