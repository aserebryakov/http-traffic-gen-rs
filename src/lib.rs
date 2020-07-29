mod config;
mod generator;
mod statistics;

use config::Config;
use config::IpList;
use std::io::prelude::*;
use log::{info, debug, trace};
extern crate log;
use std::net::TcpStream;
use std::str::FromStr;
use generator::RandomRequestGenerator;
use statistics::Statistics;


fn send_request(request: &str, address: &str, stats: &mut Statistics) {
    let buffer_size = 1024;
    stats.connection_attempt();

    let mut stream = TcpStream::connect(address).unwrap();
    let mut response = vec![0;buffer_size];

    let write_result = stream.write(&request.as_bytes());

    if write_result.is_err() {
        stats.connection_failed();
        return ()
    }

    debug!("{} error", write_result.is_err());

    let read_result = stream.read(response.as_mut_slice());
    debug!("{} error", read_result.is_err());

    if read_result.is_err() {
        stats.connection_failed();
        return ()
    }

    let response_string = String::from_utf8(response).unwrap();

    stats.count_response(response_string.lines().next().unwrap().to_string());
    debug!("Response : {}", response_string.lines().next().unwrap().to_string());
    trace!("First {} bytest of response:\n{}\n", buffer_size, response_string);
}

pub fn run(config_path: &str) -> Result<(), std::io::Error> {
    let config = Config::read_from_file(config_path).unwrap();
    info!("Config content:\n{:?}", config);

    let generator = RandomRequestGenerator {
        config: config.clone(),
        ips: IpList::from_str(config.network.as_str()).unwrap(),
    };

    let mut stats = Statistics::new();
    loop {
        send_request(generator.generate_request().as_str(), config.target.as_str(), &mut stats);
        print!("Statistics\n{}\n", stats);
    }
}
