use crate::config::*;
use std::collections::HashMap;

pub enum Command {
    PORT(u32),
    DIRECTORY(String),
    THREADS(u32),
    INVALID
}

pub enum Description {
    OPTIONAL(&'static str),
    MANDATORY(&'static str)
}

pub fn help_msg() {
    let descriptions = HashMap::from([
        ("--port 6969", Description::OPTIONAL("Host your files in a specific port. If undefined, it's 80.")),
        ("--directory /path/to/dir", Description::MANDATORY("If this argument is not provided and there's no configuration file providing it, XWaiter won't start. This argument can be shortened as --d.")),
        ("--threads 4", Description::OPTIONAL("The amount of threads you want to use. If not provided, XWaiter will be single-threaded.")),
    ]);

    for (argument, description) in descriptions {
        match description {
            Description::OPTIONAL(desc) => {
                println!("\n(optional) \t{}\n          \t{}\n", argument, desc)
            },

            Description::MANDATORY(desc) =>{
                println!("\n\t        {}\n          \t{}\n", argument, desc)
            }
        }
    }

    let mut example_config = SessionConfig::new();
    example_config.set_directory("/path/to/dir");
    example_config.set_port(6969);
    
    println!("\nArguments not marked as optional must be provided either by a configuration file or command line.\nTip: You can override these arguments by using a configuration file.\n\n## xwaiter.config.json - Example\n\n{}\n\n$ xwaiter", serde_json::to_string_pretty(&example_config).unwrap());
}

pub fn check_type(operator: &str, operand: &str) -> Command {
    // println!("{}", operand);
    match operator {
        "--port" => Command::PORT(String::from(operand).parse().expect("The provided value is not a valid port")),
        "--threads" => Command::THREADS(String::from(operand).parse().expect("The provided value is not a valid thread number")),
        "--d" => Command::DIRECTORY(String::from(operand)),
        "--directory" => Command::DIRECTORY(String::from(operand)),
        _ => {
            println!("Invalid operation. Here's a list of operations and their values:");
            help_msg();
            Command::INVALID
        }
    }
}

// This is the sexiest Rust function I ever wrote
pub fn parse_config(args: Vec<String>) -> Result<SessionConfig, ()> {
    let mut session_config: SessionConfig = SessionConfig::new();

    match session_config.set_from_config_file() {
        Ok(_) => {},
        Err(()) => return Err(())
    }

    let mut index: usize = 1;
    let exception: Result<(), ()> =  loop {
        if index >= args.len() { 
            // println!("(optional) \t{}\t{}", argument, desc)
            
            break Ok(()) 
        }

        let operator: &String = match args.get(index) {
            Some(argument) => argument,
            None => {
                //println!("Loop broke at index: {}", index);
                break Ok(())
            }
        };

        let operand: &String = match args.get(index + 1) {
            Some(argument) => argument,
            None => {
                //println!("Loop broke at index: {}", index);
                break Ok(())
            }
        };

        //println!("TRTT\n-> {} {}", operator, operand);
        match check_type(operator.as_str(), operand.as_str()) {
            Command::PORT(port) => {
                session_config.set_port(port).unwrap();
            },
            Command::DIRECTORY(directory) => {
                session_config.set_directory(directory.as_str()).unwrap();
            },
            Command::THREADS(threads) => {
                session_config.set_threads(threads).unwrap();
            },
            Command::INVALID => break Err(())
        }

        index += 2;
    };

    Ok(session_config)
}