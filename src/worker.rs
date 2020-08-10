extern crate log;

use log::{debug, trace};
use std::sync::mpsc::Sender;
use crate::generator::RandomRequestGenerator;
use crate::statistics::StatisticsUpdate;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Worker {
    stats_sender: Sender<StatisticsUpdate>,
    generator: RandomRequestGenerator,
}

impl Worker {
    pub fn new(stats_sender: Sender<StatisticsUpdate>, generator: RandomRequestGenerator) -> Worker {
        Worker {
            stats_sender,
            generator
        }
    }

    pub fn run(&self) {
        loop {
            if let Err(_) = self.send_request(
                self.generator.generate_request().as_str(),
                self.generator.config.target.as_str(), &self.stats_sender) {
                self.stats_sender.send(StatisticsUpdate::Error).unwrap();
            }
        }
    }

    fn send_request(&self, request: &str, address: &str, stats: &Sender<StatisticsUpdate>) -> Result<(), std::io::Error> {
        stats.send(StatisticsUpdate::ConnectionAttempt).unwrap();
        let mut stream = TcpStream::connect(address)?;

        let _ = stream.write(&request.as_bytes())?;
        stats.send(StatisticsUpdate::Sent).unwrap();

        let buffer_size = 1024;
        let mut response = vec![0;buffer_size];

        let _ = stream.read(response.as_mut_slice())?;

        let response_string = String::from_utf8(response).unwrap();
        stats.send(StatisticsUpdate::Received(response_string.lines().next().unwrap().to_string())).unwrap();

        debug!("Response : {}", response_string.lines().next().unwrap().to_string());
        trace!("First {} bytest of response:\n{}\n", buffer_size, response_string);

        Ok(())
    }
}
