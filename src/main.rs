use std::io::prelude::*;
use std::fs;
use log::{info, trace};
use cidr::{Cidr, Ipv4Cidr};
use std::str::FromStr;
use rand::seq::IteratorRandom;
use serde_derive::Deserialize;
extern crate log;
use std::net::TcpStream;

#[derive(Clone)]
struct IpList {
    ips: Vec<String>,
}

impl FromStr for IpList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ips = s.parse::<Ipv4Cidr>().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        Ok(IpList{ips})
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    network: String,
    target: String,
    methods: Vec<String>,
    uris: Vec<String>,
}

struct RandomRequestGenerator {
    config: Config,
    ips: IpList,
}

impl RandomRequestGenerator {
    fn generate_request(&self) -> String {
        let mut rng = rand::thread_rng();
        let method = self.config.methods.iter().choose(&mut rng).unwrap();
        let uri = self.config.uris.iter().choose(&mut rng).unwrap();
        let ip = self.ips.ips.iter().choose(&mut rng).unwrap();
        let request = format!("{} {} HTTP/1.1\r\nhost: localhost\r\nX-Forwarded-For: {}\r\n\r\n", method, uri, ip);
        trace!("Request:\n{}", request);
        request
    }
}

fn send_request(request: &str, address: &str) {
    let mut stream = TcpStream::connect(address).unwrap();
    let mut response = vec![0;256];

    stream.write(&request.as_bytes()).unwrap();
    stream.read(response.as_mut_slice()).unwrap();

    let response_string = String::from_utf8(response).unwrap();

    match (response_string.find("403 Forbidden")) {
        Some(_) => println!("BLOCKED"),
        None => println!("Response:\n{}\n", response_string),
    }
}

fn read_config(config_path: &str) -> Result<Config, ()> {
    let config = fs::read_to_string(config_path).unwrap();
    let parsed_config : Config = toml::from_str(config.as_str()).unwrap();
    Ok(parsed_config)
}

fn main() {
    env_logger::init();
    let config = read_config("config.toml").unwrap();
    info!("Config content:\n{:?}", config);

    let generator = RandomRequestGenerator {
        config: config.clone(),
        ips: IpList::from_str(config.network.as_str()).unwrap(),
    };

    loop {
        send_request(generator.generate_request().as_str(), config.target.as_str());
    }
}
