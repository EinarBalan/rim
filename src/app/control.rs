use crossterm::{
    execute,
    event::{poll, read, Event, KeyCode},
    cursor,
    Result,
};
use std::{
    io::{Stdout},
    time::Duration,
};

pub struct Position {
    row: u32,
    col: u32
}

pub fn event_loop(stdout: &Stdout) -> Result<()> {
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

fn handle_key(mut stdout: &Stdout, code: KeyCode) {
    match code {
        KeyCode::Left => { execute!(stdout, cursor::MoveLeft(1)).unwrap()},
        KeyCode::Right => { execute!(stdout, cursor::MoveRight(1)).unwrap() },
        KeyCode::Up => { execute!(stdout, cursor::MoveUp(1)).unwrap() },
        KeyCode::Down => { execute!(stdout, cursor::MoveDown(1)).unwrap() },
        _ => return (),
    }
}