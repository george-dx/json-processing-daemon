use crate::api::fetch_data;
use crate::traits::DataProcessor;

pub struct Worker<T> {
    pub data_source: T,
}

impl<T> Worker<T> {
    pub fn new(data_source: T) -> Self {
        Worker {data_source}
    }
}

impl DataProcessor for Worker<String> {
    fn description(&self) -> String {
        format!("Worker with data source: {}", self.data_source)
    }
    async fn process(&self) -> Result<(), reqwest::Error> {
        println!("Fetching data from: {}", self.data_source);
        let data = fetch_data(&self.data_source).await?;
        println!("Received {} monarchs:", data.len());
        for monarch in data {
            println!("{} ({}) - {}", monarch.Name, monarch.House, monarch.Reign);
        }
        Ok(())
    }
}

impl<T> Worker<T> where T: AsRef<str>, {
    pub fn display_data_source(&self) {
        println!("Data source: {}", self.data_source.as_ref());
    }
}

impl<T> Worker<T> where T: Default,
{
    pub fn default_worker() -> Self {
        Worker {
            data_source: T::default(),
        }
    }
}

impl Default for String {
    fn default() -> Self {
        "https://mysafeinfo.com/api/data?list=englishmonarchs&format=json".to_string()
    }
}
