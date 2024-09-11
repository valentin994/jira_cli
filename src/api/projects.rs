use crate::models::project::Projects;

pub async fn get_projects(
    client: reqwest::Client,
    api_key: String,
    page_number: u8,
) -> Result<Projects, Box<dyn std::error::Error>> {
    let url = format!(
        "https://irobot.atlassian.net/rest/api/3/project/search?maxResults=50&startAt={}",
        page_number * 50
    );
    let body = client
        .get(url)
        .basic_auth("vvareskic@irobot.com", Some(api_key))
        .send()
        .await?
        .text()
        .await?;
    let v: Projects = serde_json::from_str(&body)?;
    Ok(v)
}
