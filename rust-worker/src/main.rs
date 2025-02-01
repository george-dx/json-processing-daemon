mod api;
mod daemon;
mod models;
mod traits;
mod worker;

use crate::daemon::Daemon;
use crate::models::ApiUrl;
use crate::worker::DataType;
use worker::Worker;

#[tokio::main]
async fn main() {
    let monarch_worker = Worker::new(
        ApiUrl::new("https://mysafeinfo.com/api/data?list=englishmonarchs&format=json".to_string()),
        DataType::Monarch,
    );

    let president_worker = Worker::new(
        ApiUrl::new("https://mysafeinfo.com/api/data?list=presidents&format=json".to_string()),
        DataType::President,
    );

    let daemon = Daemon::new(monarch_worker, president_worker);
    if let Err(error) = daemon.run().await {
        eprintln!("Error: {}", error);
    }
}
