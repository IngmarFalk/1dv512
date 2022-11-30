use clap::Parser;
use core::config::Config;
use std::sync::Arc;

fn main() {
    dotenv::dotenv().ok();

    let config = Arc::new(Config::parse());

    println!("input: {}", config.input);
    println!("output: {}", config.output);
}
