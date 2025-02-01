mod api;
mod models;
mod traits;
mod worker;

use crate::models::ApiUrl;
use crate::traits::DataProcessor;
use worker::Worker;

#[tokio::main]
async fn main() {
    let worker = Worker::new(ApiUrl::new(
        "https://mysafeinfo.com/api/data?list=englishmonarchs&format=json".to_string(),
    ));

    println!("{}", worker.description());
    if let Err(e) = worker.process().await {
        eprintln!("Error: {}", e);
    }

    worker.display_data_source();

    let default_worker: Worker<ApiUrl> = Worker::default_worker();
    println!("Default worker data source: {}", default_worker.data_source);
}
