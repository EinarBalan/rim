pub mod config;
mod editor;
mod display;
pub mod vector;

use config::Config;
use editor::Editor;
use crossterm::Result;
use display::Display;

use std::{
    io::stdout,
    fs,
};

pub fn run(config: &Config) -> Result<()> {
    let stdout = stdout();

    // initialize display with content from file
    let content = fs::read_to_string(&config.file_name)
        .unwrap_or(String::from(""));
    let display = Display::new(stdout, &content, config);
    let mut editor = Editor::new(display);

    // run event loop 
    editor.event_loop()?;

    Ok(())
}
