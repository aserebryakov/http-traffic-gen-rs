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

use std::sync::mpsc::{Sender, channel};
use std::thread;

enum StatisticsUpdate {
    ConnectionAttempt,
    Sent,
    Received(String),
    Error
}

fn send_request(request: &str, address: &str, stats: &Sender<StatisticsUpdate>) {
    let buffer_size = 1024;
    stats.send(StatisticsUpdate::ConnectionAttempt).unwrap();

    let mut stream = TcpStream::connect(address).unwrap();
    let mut response = vec![0;buffer_size];

    let write_result = stream.write(&request.as_bytes());

    if write_result.is_err() {
        stats.send(StatisticsUpdate::Error).unwrap();
        return ()
    }

    stats.send(StatisticsUpdate::Sent).unwrap();

    let read_result = stream.read(response.as_mut_slice());

    if read_result.is_err() {
        stats.send(StatisticsUpdate::Error).unwrap();
        return ()
    }

    let response_string = String::from_utf8(response).unwrap();

    stats.send(StatisticsUpdate::Received(response_string.lines().next().unwrap().to_string())).unwrap();

    debug!("Response : {}", response_string.lines().next().unwrap().to_string());
    trace!("First {} bytest of response:\n{}\n", buffer_size, response_string);
}

fn update_statistics(stats: &mut Statistics, update: StatisticsUpdate) {
    match update {
        StatisticsUpdate::ConnectionAttempt => stats.connection_attempt(),
        StatisticsUpdate::Sent => stats.request_sent(),
        StatisticsUpdate::Received(response) => stats.count_response(response),
        StatisticsUpdate::Error => stats.connection_failed(),
    }
}

pub fn run(config_path: &str) -> Result<(), std::io::Error> {
    let config = Config::read_from_file(config_path).unwrap();
    info!("Config content:\n{:?}", config);

    let generator = RandomRequestGenerator {
        config: config.clone(),
        ips: IpList::from_str(config.network.as_str()).unwrap(),
    };

    let (sender, receiver) = channel();

    for _ in 0..config.worker_threads {
        let stats_sender = sender.clone();
        let generator = generator.clone();
        thread::spawn(move || {
            loop {
                send_request(generator.generate_request().as_str(), generator.config.target.as_str(), &stats_sender);
            }
        });
    }

    let mut stats = Statistics::new();
    loop {
        print!("Statistics\n{}\n", stats);
        update_statistics(&mut stats, receiver.recv().unwrap());
    }
}
