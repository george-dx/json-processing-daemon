use crate::api::fetch_data;
use crate::models::{ApiUrl, Monarch, President};

pub enum DataType {
    Monarch,
    President,
}
pub struct Worker {
    pub data_source: ApiUrl,
    pub data_type: DataType,
}

impl Worker {
    pub fn new(data_source: ApiUrl, data_type: DataType) -> Self {
        Worker {
            data_source,
            data_type,
        }
    }

    pub async fn process(&self) -> Result<usize, reqwest::Error> {
        println!("Fetching data from: {}", self.data_source);
        match self.data_type {
            DataType::Monarch => {
                let data = fetch_data::<Monarch>(&self.data_source.to_string()).await?;
                println!("Received {} monarchs:", data.len());
                for monarch in &data {
                    println!(
                        "{} ({}) - {}",
                        monarch.Name.as_deref().unwrap_or("Unknown"),
                        monarch.House.as_deref().unwrap_or("Unknown"),
                        monarch.Reign.as_deref().unwrap_or("Unknown"),
                    );
                }
                Ok(data.len())
            }
            DataType::President => {
                let data = fetch_data::<President>(&self.data_source.to_string()).await?;
                println!("Received {} presidents:", data.len());
                for president in &data {
                    println!(
                        "{} ({}) - {}",
                        president.FullName.as_deref().unwrap_or("Unknown"),
                        president.Party.as_deref().unwrap_or("Unknown"),
                        president.Terms.as_deref().unwrap_or("Unknown")
                    );
                }
                Ok(data.len())
            }
        }
    }
}
