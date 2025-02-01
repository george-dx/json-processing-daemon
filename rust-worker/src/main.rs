mod worker;
mod api;
mod models;
mod traits;

use worker::Worker;
use crate::traits::DataProcessor;

#[tokio::main]
async fn main() {
    let worker = Worker::new("https://mysafeinfo.com/api/data?list=englishmonarchs&format=json".to_string());

    println!("{}", worker.description());
    if let Err(e) = worker.process().await {
        eprintln!("Error: {}", e);
    }

    worker.display_data_source();

    let default_worker: Worker<String> = Worker::default_worker();
    println!("Default worker data source: {}", default_worker.data_source);
}