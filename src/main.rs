use std::fs;
use log::{info, trace};
use cidr::{Cidr, Ipv4Cidr};

use serde_derive::Deserialize;
extern crate log;

#[derive(Deserialize, Debug)]
struct Config {
    network: String,
    target_ip: String,
    target_port: u16,
    methods: Vec<String>,
    uris: Vec<String>
}

struct Request {
    method: String,
}

fn read_config(config_path: &str) -> Result<Config, ()> {
    let config = fs::read_to_string(config_path).unwrap();
    let parsed_config : Config = toml::from_str(config.as_str()).unwrap();
    Ok(parsed_config)
}

fn generate_request() {
}

fn main() {
    env_logger::init();
    let config = read_config("config.toml").unwrap();
    info!("Config content:\n{:?}", config);

    let network = config.network.as_str().parse::<Ipv4Cidr>().unwrap();
}
