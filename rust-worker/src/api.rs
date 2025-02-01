use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiRespose {
    pub data: String
}

pub async fn fetch_data(url: &str) -> Result<ApiRespose, Error> {
    let response = reqwest::get(url).await?.json::<ApiRespose>().await?;
    Ok(response)
}

