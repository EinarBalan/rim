use crossterm::{
    execute,
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

pub fn run() -> Result<()> {
    let mut stdout = stdout();
    let (height, width) = terminal::size()?;

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    execute!(
        stdout,
        cursor::MoveTo(height / 2, width / 2),
        Print("yo"),
        cursor::MoveDown(1), cursor::MoveLeft(10),
        Print("Press Esc to Exit"),
    )?;

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
