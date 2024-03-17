mod config;
use std::env;
use crate::config::*;

fn parse_args(args: Vec<String>) -> Result<SessionConfig, ()> {
    let mut session_config: SessionConfig = SessionConfig::new();

    for argument in args {
        println!("{}", argument);
    }

    Ok(session_config)
}

fn main() {
    let config: SessionConfig = match parse_args(env::args().collect()) {
        Ok(config) => config,
        Err(_) => return
    };

    println!("Directory: {}\nPort: {}\nThreads: {}", config.directory, config.port, config.threads);
}