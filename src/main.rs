mod config;
use crate::config::*;

fn parse_args() -> Result<SessionConfig, ()> {
    let mut session_config: SessionConfig = SessionConfig::new();

    session_config.set_port(6969).unwrap();
    session_config.set_threads(32).unwrap();
    session_config.set_directory("~/home/website").unwrap();

    Ok(session_config)
}

fn main() {
    let config: SessionConfig = match parse_args() {
        Ok(config) => config,
        Err(_) => return
    };

    println!("Directory: {}\nPort: {}\nThreads: {}", config.directory, config.port, config.threads);
}