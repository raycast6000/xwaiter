// lol

mod config;
mod parser;
mod http;

use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    env, fs
};

use crate::{
    parser::*,
    config::*,
    http::*
};

// This shit returns a response
fn process_request(directory: &String, http_request: Vec<String>) -> String {
    let mut file_path = String::new();
    file_path.push_str(directory.as_str());
    file_path.push_str(get_request_path(&http_request).as_str());

    //println!("\n\t{}\n", file_path);

    match fs::read_to_string(file_path.as_str()) {
        Ok(content) => {
            let headers = create_headers(Header::SUCCESS(content.len()));
            let new_response = Response::new(headers, content);

            return new_response.normalized()
        },

        Err(_) => {
            let headers = create_headers(Header::NOTFOUND);
            let new_response = Response::new(headers, String::new()); //Nothing this time

            return new_response.normalized()
        }
    }
}

fn handle_request(mut stream: &TcpStream, config: &SessionConfig) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Incoming request..");
    // println!("\n{:#?}", http_request);

    let response: String = process_request(&config.directory, http_request);
    
    stream.write_all(response.as_str().as_bytes()).unwrap();
    println!("Successfully responded.");
}

// srry i love splitting my code >:)
fn main() {
    let config: SessionConfig = match parse_config(env::args().collect()) {
        Ok(config) => {
            if config.directory.as_str() == "" {
                println!("Please provide valid directory to host. You can provide a directory either by configuration or argument. See this:");
                help_msg();
                return
            }

            config
        },
        Err(_) => return
    };

    
    let address = config.get_full_address();
    let listener = TcpListener::bind(address.as_str()).unwrap();

    println!("Session Info:\n\tDirectory: {}\n\tPort: 127.0.0.1:{}\n\tThreads: {}", config.directory, config.port, config.threads);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_request(&mut stream, &config);
    }

}