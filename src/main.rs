extern crate http_traffic_generator;
extern crate log;

fn main() {
    env_logger::init();
    http_traffic_generator::run("config.toml").unwrap();
}
