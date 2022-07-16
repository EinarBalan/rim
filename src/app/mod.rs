pub mod config;
mod control;
mod display;
pub mod list;

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
    let content = fs::read_to_string(&config.file_name)?;
    let mut display = Display::new(stdout, &content);

    // run event loop 
    display.event_loop()?;

    Ok(())
}
