use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
pub struct ApiUrl(String);

impl ApiUrl {
    pub fn new(url: String) -> Self {
        ApiUrl(url)
    }
}

impl Default for ApiUrl {
    fn default() -> Self {
        ApiUrl("https://mysafeinfo.com/api/data?list=englishmonarchs&format=json".to_string())
    }
}

impl fmt::Display for ApiUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for ApiUrl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct Monarch {
    pub Name: String,
    pub Country: String,
    pub House: String,
    pub Reign: String,
    pub ID: u32,
}
