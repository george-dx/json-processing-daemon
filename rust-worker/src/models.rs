use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Monarch {
    pub Name: String,
    pub Country: String,
    pub House: String,
    pub Reign: String,
    pub ID: u32,
}