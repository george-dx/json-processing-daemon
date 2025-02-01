use reqwest::Error;

pub trait DataProcessor {
    fn description(&self) -> String;
    async fn process(&self) -> Result<(), Error>;
}
