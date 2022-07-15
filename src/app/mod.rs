pub mod config;

use config::Config;

use crossterm::{
    execute, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode},
    cursor,
    style::*,
    Result,
};
use std::{
    io::{stdout, Write, Stdout, Error},
    time::Duration,
    fs,
};

pub fn run(config: &Config) -> Result<()> {
    let mut stdout = stdout();
    // let (height, width) = terminal::size()?;

    let content = fs::read_to_string(&config.file_name)?;
    let lines = content.lines();

    execute!(stdout, 
        EnterAlternateScreen,
        cursor::MoveTo(0, 0),
    )?;
    enable_raw_mode()?;

    for line in lines {
        queue!(
            stdout,
            Print(line),
            cursor::MoveToNextLine(1),
            // cursor::MoveDown(1), cursor::MoveLeft(10),
            // Print("Press Esc to Exit"),
        )?;
    }
    stdout.flush();
    

    event_loop(&stdout)?;

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())

}

fn event_loop(stdout: &Stdout) -> Result<()> {
    loop {
        if poll(Duration::from_millis(1_000))? {
            match read()? {
                Event::Key(key) => { 
                    if key.code == KeyCode::Esc { break }
                    else { handle_key(stdout, key.code) }
                },
                _ => (),
            }
        } 
    }

    Ok(())
}

// returns None in order to break if Escape key is pressed
fn handle_key(mut stdout: &Stdout, code: KeyCode) {
    match code {
        KeyCode::Left => { queue!(stdout, cursor::MoveLeft(1)).unwrap()},
        KeyCode::Right => { queue!(stdout, cursor::MoveRight(1)).unwrap() },
        KeyCode::Up => { queue!(stdout, cursor::MoveUp(1)).unwrap() },
        KeyCode::Down => { queue!(stdout, cursor::MoveDown(1)).unwrap() },
        _ => return (),
    }
    stdout.flush();
}