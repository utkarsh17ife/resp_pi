// src/config.rs

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub kafka_brokers: String,
    pub kafka_topic: String,
}

pub fn load_config() -> Config {
    let config_str = fs::read_to_string("config.json").expect("Unable to read config file");
    serde_json::from_str(&config_str).expect("Unable to parse config file")
}
