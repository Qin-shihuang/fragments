use serde::Deserialize;
use std::{error::Error, net::IpAddr};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub database_url: String,
    pub public_key: Option<String>,
    pub timezone: String,
    pub author: Author,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl Config {
    pub fn new(file: &str) -> Result<Self, Box<dyn Error>> {
        let config = std::fs::read_to_string(file)?;
        let config: Config = serde_json::from_str(&config)?;
        Ok(config)
    }
}
