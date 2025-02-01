use crate::api::fetch_data;

pub struct Worker {
    pub data_source: String,
}

impl Worker {
    pub fn new(data_source: String) -> Self {
        Worker {data_source}
    }

    pub async fn process(&self) -> Result<(), reqwest::Error> {
        println!("Fetching data from: {}", self.data_source);
        let data = fetch_data(&self.data_source).await?;
        println!("Received data: {:?}", data);
        Ok(())
    }
}