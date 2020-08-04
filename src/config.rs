use std::fs;
use cidr::{Cidr, Ipv4Cidr};
use std::str::FromStr;
use serde_derive::Deserialize;

#[derive(Clone)]
pub struct IpList {
    pub ips: Vec<String>,
}

impl FromStr for IpList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ips = s.parse::<Ipv4Cidr>().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        Ok(IpList{ips})
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub network: String,
    pub target: String,
    pub methods: Vec<String>,
    pub uris: Vec<String>,
    pub worker_threads: u32,
}

impl Config {
    pub fn read_from_file(config_path: &str) -> Result<Config, ()> {
        let config = fs::read_to_string(config_path).unwrap();
        let parsed_config : Config = toml::from_str(config.as_str()).unwrap();
        Ok(parsed_config)
    }
}
