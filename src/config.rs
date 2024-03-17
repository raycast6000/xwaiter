use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SessionConfig {
    pub directory: String,
    pub port: u32,
    pub threads: u32,
}

// I love you mom

impl SessionConfig {
    // This basically creates a default configuration structure.
    pub fn new() -> Self {
        SessionConfig {
            directory: String::new(),
            port: 8080,
            threads: 1
        }
    }

    pub fn set_threads(&mut self, amount: u32) -> Result<(), ()> {
        self.threads = amount;

        Ok(())
    }

    pub fn set_port(&mut self, port: u32) -> Result<(), ()> {
        self.port = port;

        Ok(())
    }

    pub fn set_directory(&mut self, directory: &str) -> Result<(), ()> {
        self.directory.push_str(directory);

        Ok(())
    }
}