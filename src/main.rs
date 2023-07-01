use lib::run_server;
use std::env;
use std::process;

mod lib;

const DEFAULT_PORT: i32 = 5000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (ip, port) = parse_args(args);

    run_server(&ip[..], port)
}

fn parse_args(args: Vec<String>) -> (String, i32) {
    if args.len() <= 2 || args.len() > 3 {
        eprintln!("Invalid amout of Arguments");
        process::exit(1);
    }
    let ip = args[1].clone();
    let port: i32 = args
        .get(2)
        .unwrap_or(&DEFAULT_PORT.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_PORT);

    (ip, port)
}
