extern crate http_traffic_generator;
extern crate log;
use std::env;

fn print_help() {
    println!("Usage:");
    println!("  http_traffic_generator [config_file]");
    println!("");
}

fn get_config_file_name(args: Vec<String>) -> String {
    if args.len() == 1 {
        return String::from("config.toml");
    }

    args[1].clone()
}

fn main() {
    env_logger::init();
    print_help();
    let args: Vec<String> = env::args().collect();
    http_traffic_generator::run(get_config_file_name(args).as_str()).unwrap();
}
