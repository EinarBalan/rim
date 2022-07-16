use crossterm::{
    execute,
    event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent},
    cursor,
    Result, queue,
    terminal,
};
use std::{
    io::Write,
    time::Duration, 
    cmp, fs,
};

use super::display::Display;

pub fn event_loop(display: &mut Display) -> Result<()> {
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
                    else { handle_key_event(display, key_event)?; }
                },
                _ => (),
            }
        } 
    }

    Ok(())
}

fn handle_key_event(display: &mut Display, event: KeyEvent) -> Result<()> {
    let stdout = &mut display.stdout;

    match event {
        // standard controls (no modifiers applied)
        KeyEvent { modifiers: KeyModifiers::NONE, code } => {
            match code {
                KeyCode::Left => { move_cursor(display, 0, -1) },
                KeyCode::Right => { move_cursor(display, 0, 1) },
                KeyCode::Up => { move_cursor(display, -1, 0) },
                KeyCode::Down => { move_cursor(display, 1, 0) },
                KeyCode::Backspace => { 
                    // Backspace
                    delete(display, 1)?;
                    display.refresh()
                }
                KeyCode::Char(c) => {
                    // type characters
                    insert(display, c)?;
                    display.refresh()
                }
                _ => return Ok(()),
            }
        },

        // CTRL modifer
        KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
            let width = terminal::size()?.0 as i32;
            match code {
                KeyCode::Char('b') => { move_cursor(display, 0, -1) },
                KeyCode::Char('f') => { move_cursor(display, 0, 1) },
                KeyCode::Char('p') => { move_cursor(display, -1, 0) },
                KeyCode::Char('n') => { move_cursor(display, 1, 0) },
                KeyCode::Char('a') => { execute!(stdout, cursor::MoveToColumn(0)) },
                KeyCode::Char('e') => { move_cursor(display, 0, width)},
                KeyCode::Char('d') => { 
                    // Delete
                    delete(display, 0)?;
                    display.refresh()
                }
                KeyCode::Char('k') => { 
                    // Kill to end of line
                    kill(display)?;
                    display.refresh()
                }
                KeyCode::Char('s') => {
                    // Save edits to file 
                    save(display)
                }
                _ => return Ok(()),
            }
        },

        _ => Ok(())
    }

}

/// Move cursor in the y rows and x columns if possible.
/// Will not move if outside the bounds of the text.
fn move_cursor(display: &mut Display,  y: i32, x: i32) -> Result<()> {
    let lines = &mut display.lines;
    let stdout = &mut display.stdout;

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
fn delete(display: &mut Display, offset: i32) -> Result<()> {
    let lines = &mut display.lines;

    let (cur_col, cur_row) = cursor::position()?;
    let cur_line = &mut lines[cur_row as usize];
    
    let pos = (cur_col as i32 - offset) as usize;
    if pos < cur_line.len() {
        cur_line.remove(pos);
        if offset > 0 {
            move_cursor(display, 0, -(offset as i32))?;
        }
    }

    Ok(())
}

/// Delete character starting at cursor until end of line
fn kill(display: &mut Display) -> Result<()> {
    let lines = &mut display.lines;

    let (cur_col, cur_row) = cursor::position()?;
    let cur_line = &mut lines[cur_row as usize];
    
    let pos = cur_col as usize;
        if pos < cur_line.len() {
        cur_line.replace_range(pos.., "");
    }

    Ok(())
}

fn insert(display: &mut Display, c: char) -> Result<()> {
    let lines = &mut display.lines;
    
    let (cur_col, cur_row) = cursor::position()?;
    let cur_line = &mut lines[cur_row as usize];
    
    let pos = cur_col as usize;
    if pos < cur_line.len() { cur_line.insert(pos, c); }
    else { cur_line.push(c); }
    move_cursor(display, 0, 1)?;

    Ok(())
}

fn save(display: &mut Display) -> Result<()> {
    let lines = &mut display.lines;
    let file_name = &mut display.file_name;
    let mut content = String::new();
    for line in lines {
        content.push_str(&format!("{}\n", line)[..]);
    }

    fs::write(file_name, content)?;

    Ok(())
}
