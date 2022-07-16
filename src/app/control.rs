use crossterm::{
    execute,
    event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent},
    cursor,
    Result, queue,
    terminal,
};
use std::{
    io::{Stdout, Write},
    time::Duration, 
    cmp,
};

use super::display::{Display, self};

pub fn event_loop(display: &mut Display) -> Result<()> {
    let stdout = &display.stdout;
    let lines = &mut display.lines;

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
                    else { handle_key_event(stdout, lines, key_event)?; }
                },
                _ => (),
            }
        } 
    }

    Ok(())
}

fn handle_key_event(mut stdout: &Stdout, lines: &mut Vec<String>, event: KeyEvent) -> Result<()> {
    match event {
        // standard controls (no modifiers applied)
        KeyEvent { modifiers: KeyModifiers::NONE, code } => {
            match code {
                KeyCode::Left => { move_cursor(stdout, lines, 0, -1) },
                KeyCode::Right => { move_cursor(stdout, lines, 0, 1) },
                KeyCode::Up => { move_cursor(stdout, lines, -1, 0) },
                KeyCode::Down => { move_cursor(stdout, lines, 1, 0) },
                KeyCode::Backspace => { 
                    delete(stdout, 1, lines)?;
                    Display::refresh(stdout, lines)
                }
                KeyCode::Char(c) => {
                    insert(stdout, c, lines)?;
                    Display::refresh(stdout, lines)
                }
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
                KeyCode::Char('e') => { move_cursor(stdout, lines, 0, width)},
                KeyCode::Char('d') => { 
                    delete(stdout, 0, lines)?;
                    Display::refresh(stdout, lines)
                }
                _ => return Ok(()),
            }
        },

        _ => Ok(())
    }

}

/// Move cursor in the y rows and x columns if possible.
/// Will not move if outside the bounds of the text.
fn move_cursor(mut stdout: &Stdout, lines: &Vec<String>,  y: i32, x: i32) -> Result<()> {
    let (cur_col, cur_row) = cursor::position().unwrap();
    let (mut new_col, new_row) = (((cur_col as i32) + x) as u16, ((cur_row as i32) + y) as u16);

    let num_rows = lines.len() as u16;
    if new_row > num_rows {
        return Ok(());
    }

    if let Some(cur_line) = lines.get(new_row as usize) {
        let num_cols = cur_line.len() as u16;
        new_col = cmp::min(new_col, num_cols);
    
        queue!(stdout, cursor::MoveTo(new_col, new_row))?;
        stdout.flush()?;
    }

    Ok(())
}

/// Delete character at left directed offset from cursor on current row if possible
fn delete(mut stdout: &Stdout, offset: i32, lines: &mut Vec<String>) -> Result<()> {
    let (cur_col, cur_row) = cursor::position()?;
    let cur_line = &mut lines[cur_row as usize];
    
    let pos = (cur_col as i32 - offset) as usize;
    if pos < cur_line.len() {
        cur_line.remove(pos);
        if offset > 0 {
            move_cursor(stdout, lines, 0, -(offset as i32))?;
        }
    }

    Ok(())
}

fn insert(mut stdout: &Stdout, c: char, lines: &mut Vec<String>) -> Result<()> {
    let (cur_col, cur_row) = cursor::position()?;
    let cur_line = &mut lines[cur_row as usize];
    
    let pos = cur_col as usize;
    if pos < cur_line.len() { cur_line.insert(pos, c); }
    else { cur_line.push(c); }
    move_cursor(stdout, lines, 0, 1)?;

    Ok(())
}
