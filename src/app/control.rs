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

pub fn event_loop(display: &mut Display) -> Result<()> {
    let stdout = &display.stdout;
    let mut cursor = display.lines.cursor_front_mut();

    loop {
        // listen for key
        if poll(Duration::from_millis(1000))? {
            match read()? {
                Event::Key(key_event) => { 
                    // exit program on escape or Ctrl-X
                    if key_event.code == KeyCode::Esc || 
                    (key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('x')) { 
                        break 
                    }
                    else { handle_key_event(stdout, &mut cursor, key_event)?; }
                },
                _ => (),
            }
        } 
    }

    Ok(())
}

fn handle_key_event(mut stdout: &Stdout, cursor: &mut CursorMut<String>, event: KeyEvent) -> Result<()> {
    match event {
        // standard controls (no modifiers applied)
        KeyEvent { modifiers: KeyModifiers::NONE, code } => {
            match code {
                KeyCode::Left => { move_cursor(stdout, cursor , 0, -1) },
                KeyCode::Right => { move_cursor(stdout, cursor , 0, 1) },
                KeyCode::Up => { move_cursor(stdout, cursor , -1, 0) },
                KeyCode::Down => { move_cursor(stdout, cursor , 1, 0) },
                KeyCode::Delete => {
                    let (cur_col, cur_row) = cursor::position()?;

                    Display::refresh_after(cursor)?;
                    Ok(())
                }
                _ => return Ok(()),
            }
        },

        // CTRL modifer
        KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
            let width = terminal::size()?.0 as i32;
            match code {
                KeyCode::Char('b') => { move_cursor(stdout, cursor , 0, -1) },
                KeyCode::Char('f') => { move_cursor(stdout, cursor , 0, 1) },
                KeyCode::Char('p') => { move_cursor(stdout, cursor , -1, 0) },
                KeyCode::Char('n') => { move_cursor(stdout, cursor , 1, 0) },
                KeyCode::Char('a') => { execute!(stdout, cursor::MoveToColumn(0)) },
                KeyCode::Char('e') => { move_cursor(stdout, cursor , 0, width)}
                _ => return Ok(()),
            }
        },

        _ => Ok(())
    }

}

/// Move cursor in the y rows and x columns if possible.
/// Will not move if outside the bounds of the text.
fn move_cursor(mut stdout: &Stdout, cursor: &mut CursorMut<String>,  y: i32, x: i32) -> Result<()> {
    let (cur_col, cur_row) = cursor::position().unwrap();
    let (mut new_col, mut new_row) = (cur_col, cur_row);

    if y > 0 {
        // MOVE DOWN
        if let Some(_) = cursor.peek_next() {
            cursor.move_next();
            new_row += y as u16;
        }
    } 
    else if y < 0 {
        // MOVE UP
        if let Some(_) = cursor.peek_prev() {
            cursor.move_prev();
            new_row -= y.abs() as u16;
        }
    }

    let cur_line = cursor.current().unwrap();
    let num_cols = cur_line.len() as u16;
    new_col = ((new_col as i32) + x) as u16;
    new_col = cmp::min(new_col, num_cols);

    queue!(stdout, cursor::MoveTo(new_col as u16, new_row as u16))?;
    stdout.flush()?;

    Ok(())
}