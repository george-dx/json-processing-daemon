use serde::de::DeserializeOwned;

pub async fn fetch_data<T>(url: &str) -> Result<Vec<T>, reqwest::Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await?;
    let raw_response = response.text().await?; // Get the raw response as a string
                                               // println!("Raw API response: {}", raw_response); // Log the raw response
    let data: Vec<T> = serde_json::from_str(&raw_response).unwrap(); // Parse the JSON
    Ok(data)
}
