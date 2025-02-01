use crate::models::Monarch;

pub async fn fetch_data(url: &str) -> Result<Vec<Monarch>, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let raw_response = response.text().await?; // Get the raw response as a string
    println!("Raw API response: {}", raw_response); // Log the raw response
    let data: Vec<Monarch> = serde_json::from_str(&raw_response).unwrap(); // Parse the JSON
    Ok(data)
}
