use crossterm::{
    execute,
    event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent},
    cursor,
    Result, queue,
    terminal,
};
use std::{
    io::{Stdout, Write},
    time::Duration, collections::LinkedList,
    cmp,
};

use super::display::Display;

pub fn event_loop(display: &Display) -> Result<()> {
    let stdout = &display.stdout;

    loop {
        if poll(Duration::from_millis(1000))? {
            match read()? {
                Event::Key(key_event) => { 
                    // exit program on escape or Ctrl-X
                    if key_event.code == KeyCode::Esc || 
                    (key_event.code == KeyCode::Char('x') && key_event.modifiers == KeyModifiers::CONTROL) { 
                        break 
                    }
                    else { handle_key_event(stdout, &display.lines, key_event); }
                },
                _ => (),
            }
        } 
    }

    Ok(())
}

fn handle_key_event(mut stdout: &Stdout, mut lines: &Vec<LinkedList<char>>, event: KeyEvent) -> Result<()> {
    match event {
        // standard controls (no modifiers applied)
        KeyEvent { modifiers: KeyModifiers::NONE, code } => {
            match code {
                KeyCode::Left => { move_cursor(stdout, lines, 0, -1) },
                KeyCode::Right => { move_cursor(stdout, lines, 0, 1) },
                KeyCode::Up => { move_cursor(stdout, lines, -1, 0) },
                KeyCode::Down => { move_cursor(stdout, lines, 1, 0) },
                _ => return Ok(()),
            }
        },

        // CTRL modifer
        KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
            let width = terminal::size()?.0 as i32;
            match code {
                KeyCode::Char('b') => { move_cursor(stdout, lines, 0, -1) },
                KeyCode::Char('f') => { move_cursor(stdout, lines, 0, 1) },
                KeyCode::Char('p') => { move_cursor(stdout, lines, -1, 0) },
                KeyCode::Char('n') => { move_cursor(stdout, lines, 1, 0) },
                KeyCode::Char('a') => { execute!(stdout, cursor::MoveToColumn(0)) },
                KeyCode::Char('e') => { move_cursor(stdout, lines, 0, width)}
                _ => return Ok(()),
            }
        },

        _ => Ok(())
    }
}

/// Move cursor in the y rows and x columns if possible
fn move_cursor(mut stdout: &Stdout, mut lines: &Vec<LinkedList<char>>, y: i32, x: i32) -> Result<()> {
    let (cur_col, cur_row) = cursor::position().unwrap();
    let (new_row, new_col) = ((cur_row as i32 + y) as usize, (cur_col as i32 + x) as usize);
    let (num_rows, num_cols) = (lines.len(), (&lines)[new_row as usize].len());

    // ensure cursor is within bounds of text
    if new_row < 0 || new_row >= num_rows { return Ok(()); }
    let new_col = cmp::min(new_col, num_cols);

    queue!(stdout, cursor::MoveTo(new_col as u16, new_row as u16));
    stdout.flush()?;

    Ok(())
}