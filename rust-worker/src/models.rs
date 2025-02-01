#![allow(non_snake_case)]

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
    pub Name: Option<String>,
    pub Country: Option<String>,
    pub House: Option<String>,
    pub Reign: Option<String>,
    pub ID: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct President {
    pub President: Option<u32>,
    pub ID: Option<u32>,
    pub FullName: Option<String>,
    pub Party: Option<String>,
    pub Terms: Option<String>,
}
