pub mod config;
mod control;
mod list;

use config::Config;

use crossterm::{
    execute, queue,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    cursor,
    style::*,
    Result,
};
use std::{
    io::{stdout, Write},
    fs, collections::LinkedList,
};

pub fn run(config: &Config) -> Result<()> {
    let mut stdout = stdout();

    let content = fs::read_to_string(&config.file_name)?;
    let lines: Vec<&str> = content.lines().collect();
    let lines = list::from_vec(&lines);

    execute!(stdout, 
        EnterAlternateScreen,
        cursor::MoveTo(0, 0),
    )?;
    enable_raw_mode()?;

    for line in lines {
        queue!(
            stdout,
            Print(list::display(&line)),
            cursor::MoveToNextLine(1),
        )?;
    }
    stdout.flush()?;

    control::event_loop(&stdout)?;

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
