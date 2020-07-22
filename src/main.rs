use std::fs;
use log::{info, warn, error};

//#[macro_use]
extern crate log;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    network: String,
    port: u16,
    methods: Vec<String>,
    uris: Vec<String>
}

fn main() {
    env_logger::init();
    let config = fs::read_to_string("config.toml").unwrap();
    let parsed_config : Config = toml::from_str(config.as_str()).unwrap();
    info!("Config content:\n{:?}", parsed_config);
}
