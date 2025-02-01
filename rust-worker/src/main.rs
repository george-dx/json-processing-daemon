mod api;
mod models;
mod traits;
mod worker;

use crate::models::ApiUrl;
use crate::traits::DataProcessor;
use worker::Worker;
use crate::worker::DataType;

#[tokio::main]
async fn main() {
    let monarch_worker = Worker::new(
        ApiUrl::new("https://mysafeinfo.com/api/data?list=englishmonarchs&format=json".to_string()),
        DataType::Monarch,
    );

    let president_worker = Worker::new(ApiUrl::new("https://mysafeinfo.com/api/data?list=presidents&format=json".to_string()),
    DataType::President,);

    println!("{}", monarch_worker.description());
    if let Err(e) = monarch_worker.process().await {
        eprintln!("Error: {}", e);
    }

    // Use the DataProcessor trait for presidents
    println!("{}", president_worker.description());
    if let Err(e) = president_worker.process().await {
        eprintln!("Error: {}", e);
    }
}
