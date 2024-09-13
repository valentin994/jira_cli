use crate::models::board::{Board, Boards};

pub async fn get_boards(
    client: reqwest::Client,
    domain: String,
    user: String,
    api_key: String,
    page: u8,
) -> Result<Boards, Box<dyn std::error::Error>> {
    let start_at = if page <= 1 { 0 } else { page as u16 * 50 };
    let url = format!("https://{domain}/rest/agile/1.0/board?startAt={start_at}",);
    let body = client
        .get(url)
        .basic_auth(user, Some(api_key))
        .send()
        .await?
        .text()
        .await?;
    let v: Boards = serde_json::from_str(&body)?;
    Ok(v)
}

pub async fn get_board(
    client: reqwest::Client,
    domain: &String,
    user: &String,
    api_key: &String,
    id: u16,
) -> Result<Board, Box<dyn std::error::Error>> {
    let url = format!("https://{domain}/rest/agile/1.0/board/{id}");
    let body = client
        .get(url)
        .basic_auth(user, Some(api_key))
        .send()
        .await?
        .text()
        .await?;
    let v: Board = serde_json::from_str(&body)?;
    Ok(v)
}
