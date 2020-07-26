mod config;
use config::Config;
use config::IpList;
use std::io::prelude::*;
use log::{info, trace};
use rand::seq::IteratorRandom;
extern crate log;
use std::net::TcpStream;
use std::str::FromStr;

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

    match response_string.find("403 Forbidden") {
        Some(_) => println!("FORBIDDEN"),
        None => println!("Response:\n{}\n", response_string),
    }
}

pub fn run(config_path: &str) -> Result<(), std::io::Error> {
    let config = Config::read_from_file(config_path).unwrap();
    info!("Config content:\n{:?}", config);

    let generator = RandomRequestGenerator {
        config: config.clone(),
        ips: IpList::from_str(config.network.as_str()).unwrap(),
    };

    loop {
        send_request(generator.generate_request().as_str(), config.target.as_str());
    }
}
