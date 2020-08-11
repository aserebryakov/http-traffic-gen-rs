extern crate log;

mod config;
mod generator;
mod statistics;
mod worker;

use config::Config;
use config::IpList;
use std::str::FromStr;
use generator::RandomRequestGenerator;
use statistics::{Statistics, StatisticsUpdate};
use log::info;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use worker::Worker;
use std::time::{SystemTime, Duration};

fn run_stats_updater(receiver: Receiver<StatisticsUpdate>) {
    let mut stats = Statistics::new();
    let mut now = SystemTime::now();
    let start = SystemTime::now();

    loop {
        stats.update(receiver.recv().unwrap());
        if now.elapsed().unwrap() > Duration::new(1, 0) {
            now = SystemTime::now();
            println!("Statistics");
            println!("==========");
            print!("\nElapsed time {}s\n{}\n", start.elapsed().unwrap().as_secs(), stats);
        }
    }
}

fn run_workers(count: u32, generator: RandomRequestGenerator, sender: Sender<StatisticsUpdate>) {
    for _ in 0..count {
        let sender = sender.clone();
        let generator = generator.clone();
        thread::spawn(move || {
            Worker::new(sender, generator).run();
        });
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

    run_workers(config.worker_threads, generator, sender);
    run_stats_updater(receiver);

    Ok(())
}
