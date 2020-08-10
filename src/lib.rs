extern crate log;

mod config;
mod generator;
mod statistics;
mod worker;

use config::Config;
use config::IpList;
use std::str::FromStr;
use generator::RandomRequestGenerator;
use statistics::{Statistics, update_statistics};
use log::info;

use std::sync::mpsc::channel;
use std::thread;
use worker::Worker;

pub fn run(config_path: &str) -> Result<(), std::io::Error> {
    let config = Config::read_from_file(config_path).unwrap();
    info!("Config content:\n{:?}", config);

    let generator = RandomRequestGenerator {
        config: config.clone(),
        ips: IpList::from_str(config.network.as_str()).unwrap(),
    };

    let (sender, receiver) = channel();

    for _ in 0..config.worker_threads {
        let sender = sender.clone();
        let generator = generator.clone();
        thread::spawn(move || {
            Worker::new(sender, generator).run();
        });
    }

    let mut stats = Statistics::new();
    loop {
        print!("Statistics\n{}\n", stats);
        update_statistics(&mut stats, receiver.recv().unwrap());
    }
}
