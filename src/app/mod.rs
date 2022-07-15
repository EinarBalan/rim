pub mod config;

use config::Config;

use crossterm::{
    execute, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode, KeyEvent},
    cursor,
    style::*,
    Result,
};
use std::{
    io::{stdout, Write},
    time::Duration,
    fs,
};

pub fn run(config: &Config) -> Result<()> {
    let mut stdout = stdout();
    let (height, width) = terminal::size()?;

    let content = fs::read_to_string(&config.file_name)?;
    let lines = content.lines();

    execute!(stdout, 
        EnterAlternateScreen,
        cursor::MoveTo(0, 0),
    )?;
    enable_raw_mode()?;

    for line in lines {
        execute!(
            stdout,
            Print(line),
            cursor::MoveToNextLine(1),
            // cursor::MoveDown(1), cursor::MoveLeft(10),
            // Print("Press Esc to Exit"),
        )?;
    }
    

    event_loop()?;

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())

}

fn event_loop() -> Result<()> {
    loop {
        if poll(Duration::from_millis(1_000))? {
            match read()? {
                Event::Key(key_code) => {
                    if key_code == KeyCode::Esc.into() { break }
                },
                _ => (),
            }
        } 
    }

    Ok(())
}
