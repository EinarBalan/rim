use std::env;

pub struct Config {
    pub file_name: String
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_name = match args.next() {
            Some(file_name) => file_name,
            None => return Err("Missing file name")
        };

        Ok(Config { file_name })
    }
}