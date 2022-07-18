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

    // read contents from file or create new buffer
    let content = fs::read_to_string(&config.file_name)
        .unwrap_or(String::from(""));

    // initialize display with content from file
    let mut display = Display::new(stdout, &content, config);
    display.show()?;

    // run event loop 
    let mut editor = Editor::new(display);
    editor.event_loop()?;

    Ok(())
}
