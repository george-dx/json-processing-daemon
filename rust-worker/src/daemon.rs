use crate::worker::Worker;
use reqwest::Error;
use tokio::io::{self, AsyncBufReadExt};

pub struct Daemon {
    monarch_worker: Worker,
    president_worker: Worker,
}

impl Daemon {
    pub fn new(monarch_worker: Worker, president_worker: Worker) -> Self {
        Daemon {
            monarch_worker,
            president_worker,
        }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();

        println!("Daemon is running. Enter a command (type 'help' for options):");

        while let Some(line) = reader.next_line().await.unwrap() {
            match line.trim() {
                "1" => {
                    println!("Processing monarch data...");
                    let count = self.monarch_worker.process().await?;
                    if count > 10 {
                        println!("Enough monarchs to do statistics!");
                    } else {
                        println!("Not enough monarchs for statistics.");
                    }
                }
                "2" => {
                    println!("Processing president data...");
                    let count = self.president_worker.process().await?;
                    if count < 100 {
                        println!("Not ready to do any statistics yet.");
                    } else {
                        println!("Enough presidents to do statistics!");
                    }
                }
                "help" => {
                    println!("Available commands:");
                    println!("  1 - Process monarch data");
                    println!("  2 - Process president data");
                    println!("  exit - Exit the daemon");
                }
                "exit" => {
                    println!("Exiting daemon...");
                    break;
                }
                _ => {
                    println!("Unknown command. Type 'help' for options.");
                }
            }
        }
        Ok(())
    }
}
