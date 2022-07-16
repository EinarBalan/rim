use crossterm::{
    execute,
    event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent},
    cursor,
    Result, queue,
    terminal,
};
use std::{
    io::{Stdout, Write},
    time::Duration, collections::{LinkedList, linked_list::CursorMut},
    cmp,
};

use super::display::{Display, self};

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
                    else { handle_key_event(&display, key_event)?; }
                },
                _ => (),
            }
        } 
    }

    Ok(())
}

fn handle_key_event(mut display: &Display, event: KeyEvent) -> Result<()> {
    match event {
        // standard controls (no modifiers applied)
        KeyEvent { modifiers: KeyModifiers::NONE, code } => {
            match code {
                KeyCode::Left => { move_cursor(&display, 0, -1) },
                KeyCode::Right => { move_cursor(&display, 0, 1) },
                KeyCode::Up => { move_cursor(&display, -1, 0) },
                KeyCode::Down => { move_cursor(&display, 1, 0) },
                KeyCode::Delete => {
                    let (cur_col, cur_row) = cursor::position()?;

                    Ok(())
                }
                _ => return Ok(()),
            }
        },

        // CTRL modifer
        KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
            let width = terminal::size()?.0 as i32;
            match code {
                KeyCode::Char('b') => { move_cursor(&display, 0, -1) },
                KeyCode::Char('f') => { move_cursor(&display, 0, 1) },
                KeyCode::Char('p') => { move_cursor(&display, -1, 0) },
                KeyCode::Char('n') => { move_cursor(&display, 1, 0) },
                KeyCode::Char('a') => { execute!(&display.stdout, cursor::MoveToColumn(0)) },
                KeyCode::Char('e') => { move_cursor(&display, 0, width)}
                _ => return Ok(()),
            }
        },

        _ => Ok(())
    }
}

/// Move cursor in the y rows and x columns if possible
fn move_cursor(display: &Display, y: i32, x: i32) -> Result<()> {
    let mut stdout = &display.stdout;
    let lines = &display.lines;

    let (cur_col, cur_row) = cursor::position().unwrap();
    let (new_row, new_col) = ((cur_row as i32 + y) as usize, (cur_col as i32 + x) as usize);
    // let (num_rows, num_cols) = (lines.len(), (&lines)[new_row as usize].len());
    let num_rows = lines.len();

    // ensure cursor is within bounds of text
    if new_row >= num_rows { return Ok(()); }
    // let new_col = cmp::min(new_col, num_cols);

    queue!(stdout, cursor::MoveTo(new_col as u16, new_row as u16))?;
    stdout.flush()?;

    Ok(())
}