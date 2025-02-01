use crate::api::fetch_data;
use crate::models::{ApiUrl, Monarch, President};
use crate::traits::DataProcessor;

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
        Worker { data_source, data_type }
    }
}

impl DataProcessor for Worker{
    fn description(&self) -> String {
        format!("Worker with data source: {}", self.data_source)
    }
    async fn process(&self) -> Result<(), reqwest::Error> {
        println!("Fetching data from: {}", self.data_source);
        match self.data_type {
            DataType::Monarch => {
                let data = fetch_data::<Monarch>(&self.data_source.to_string()).await?;
                println!("Received {} monarchs:", data.len());
                for monarch in data {
                    println!("{} ({}) - {}", monarch.Name.unwrap_or_else(|| "Unknown".to_string()), monarch.House.unwrap_or_else(|| "Unknown".to_string()),
                    monarch.Reign.unwrap_or_else(|| "Unknown".to_string()));
                }
            }
            DataType::President => {
                let data = fetch_data::<President>(&self.data_source.to_string()).await?;
                println!("Received {} presidents:", data.len());
                for president in data {
                    println!("{} ({}) - {}", president.FullName.unwrap_or_else(|| "Unknown".to_string()),
                    president.Party.unwrap_or_else(|| "Unknown".to_string()),
                    president.Terms.unwrap_or_else(|| "Unknown".to_string()));
                }
            }
        }
        Ok(())
    }
}
