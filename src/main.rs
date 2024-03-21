// lol

mod config;
mod parser;

use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    env, fs
};

use crate::parser::*;
use crate::config::*;

fn get_request_path(http_request: &Vec<String>) -> String {
    let request_line: Vec<&str> = http_request.get(0).unwrap().split(" ").collect();
    let req_path = request_line.get(1).unwrap();

    println!("{}", req_path);
    String::from(req_path.to_owned())
}

enum Header {
    SUCCESS(usize),
    NOTFOUND
}

fn create_headers(header: Header) -> String {
    match header {
        Header::SUCCESS(length) => String::from(format!("HTTP/1.1 200 OK\r\nX-Powered-By: XWaiter\r\nContent-Length: {length}\r\n\r\n")),
        Header::NOTFOUND => String::from("HTTP/1.1 404 OK\r\nX-Powered-By: XWaiter\r\n\r\n")
    }
}

struct Response {
    headers: String,
    contents: String
}

impl Response {
    pub fn new(headers: String, contents: String) -> Self {
        Response {
            headers,
            contents
        }
    }

    // This encodes all its information in a single String.
    pub fn normalized(&self) -> String {
        let mut response = String::new();
        response.push_str(self.headers.as_str());
        response.push_str(self.contents.as_str());

        response
    }
}

// This shit returns a response
fn process_request(directory: &String, http_request: Vec<String>) -> String {
    let mut file_path = String::new();
    file_path.push_str(directory.as_str());
    file_path.push_str(get_request_path(&http_request).as_str());

    println!("\n\t{}\n", file_path);

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
    
    //let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {reqf_len}\r\nX-Powered-By: XWaiter\r\n\r\n{request_file}");
}

fn handle_request(mut stream: &TcpStream, config: &SessionConfig) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Connection established!");
    println!("\n{:#?}", http_request);

    let response: String = process_request(&config.directory, http_request);
    
    stream.write_all(response.as_str().as_bytes()).unwrap();
}

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

    
    let address = config.get_full_address();
    let listener = TcpListener::bind(address.as_str()).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_request(&mut stream, &config);
    }

    println!("Directory: {}\nPort: {}\nThreads: {}", config.directory, config.port, config.threads);
}


//lol