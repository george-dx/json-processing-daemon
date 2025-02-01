mod worker;
mod api;
mod command;
mod oracle;
mod models;

use worker::Worker;

#[tokio::main]
async fn main() {
    let worker = Worker::new("https://mysafeinfo.com/api/data?list=englishmonarchs&format=json".to_string());
    if let Err(e) = worker.process().await {
        eprintln!("Error: {}", e);
    }
}
