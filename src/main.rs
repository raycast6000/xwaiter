mod config;
mod parser;

use std::env;
use crate::parser::*;
use crate::config::*;
use std::net::TcpListener;

// srry i love splitting my code >:)
fn main() {
    let config: SessionConfig = match parse_args(env::args().collect()) {
        Ok(config) => {
            if config.directory.as_str() == "" {
                println!("Please provide valid directory to host. Here's how to do it:");
                help_msg();
                return
            }
            
            config
        },
        Err(_) => return
    };

    
    println!("Directory: {}\nPort: {}\nThreads: {}", config.directory, config.port, config.threads);
}