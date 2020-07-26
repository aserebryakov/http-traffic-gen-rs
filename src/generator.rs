extern crate log;

use crate::config::Config;
use crate::config::IpList;
use log::debug;
use rand::seq::IteratorRandom;

pub struct RandomRequestGenerator {
    pub config: Config,
    pub ips: IpList,
}

impl RandomRequestGenerator {
    pub fn generate_request(&self) -> String {
        let mut rng = rand::thread_rng();
        let method = self.config.methods.iter().choose(&mut rng).unwrap();
        let uri = self.config.uris.iter().choose(&mut rng).unwrap();
        let ip = self.ips.ips.iter().choose(&mut rng).unwrap();
        let request = format!("{} {} HTTP/1.1\r\nhost: localhost\r\nX-Forwarded-For: {}\r\n\r\n", method, uri, ip);
        debug!("Request:\n{}", request);
        request
    }
}
