pub mod config;
mod control;
mod display;
pub mod vector;

use config::Config;
use crossterm::Result;
use display::Display;

use std::{
    io::stdout,
    fs,
};

pub fn run(config: &Config) -> Result<()> {
    let stdout = stdout();

    // initialize display with content from file
    let mut content = fs::read_to_string(&config.file_name).unwrap_or_else(|_| {
        // file does not exist, so create a new buffer
        String::new()
    });
    content.push('\n');
    let mut display = Display::new(stdout, &content, &config);

    // run event loop 
    display.event_loop()?;

    Ok(())
}
