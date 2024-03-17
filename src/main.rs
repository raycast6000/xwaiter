mod config;
mod parser;

use std::env;
use crate::parser::*;
use crate::config::*;


// srry i love splitting my code >:)
fn main() {
    let config: SessionConfig = match parse_args(env::args().collect()) {
        Ok(config) => config,
        Err(_) => return
    };

    println!("Directory: {}\nPort: {}\nThreads: {}", config.directory, config.port, config.threads);
}