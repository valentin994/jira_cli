use crate::models::issue::Issues;

pub async fn get_issues(
    client: reqwest::Client,
    domain: String,
    api_key: String,
) -> Result<Issues, Box<dyn std::error::Error>> {
    let url = format!("https://{domain}/rest/agile/1.0/board/333/sprint/7281/issue");
    let body = client
        .get(url)
        .basic_auth("vvareskic@irobot.com", Some(api_key))
        .send()
        .await?
        .text()
        .await?;
    let v: Issues = serde_json::from_str(&body)?;
    Ok(v)
}
