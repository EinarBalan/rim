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
                KeyCode::Enter => {
                    split_line(display)?;
                    display.refresh()
                },
                KeyCode::Backspace => { 
                    // Backspace
                    delete(display, 1)?;
                    display.refresh()
                },
                KeyCode::Char(c) => {
                    // type characters
                    let string = String::from(c) ;
                    insert(display, &string)?;
                    display.refresh()
                },
                KeyCode::Tab => {
                    insert(display, "    ")?;
                    display.refresh()
                },
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
                },
                KeyCode::Char('k') => { 
                    // Kill to end of line
                    kill(display)?;
                    display.refresh()
                },
                KeyCode::Char('s') => {
                    // Save edits to file 
                    save(display)
                },
                _ => return Ok(()),
            }
        },

        // Shift modifier
        KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {
            match code {
                KeyCode::Char(c) => { 
                    insert(display, &c.to_uppercase().to_string())?;
                    display.refresh()
                },
                _ => return Ok(()),
            }
        },

        _ => Ok(())
    }

}

fn cursor_pos_usize() -> (usize, usize) {
    let (x, y) = cursor::position().unwrap();
    (x as usize, y as usize)
}

/// Move cursor y rows and x columns if possible.
/// Will not move if outside the bounds of the text.
fn move_cursor(display: &mut Display,  y: i32, x: i32) -> Result<()> {
    let lines = &mut display.lines;
    let stdout = &mut display.stdout;

    let (cur_col, cur_row) = cursor::position()?;
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

/// Move cursor to position row y column x
/// If either == -1, stay in same position
fn move_cursor_abs(display: &mut Display,  y: Option<u16>, x: Option<u16>) -> Result<()> {
    let (cur_col, cur_row) = cursor::position()?;

    let move_y = y.unwrap_or_else(|| { cur_row });
    let move_x = x.unwrap_or_else(|| { cur_col });

    execute!(display.stdout, cursor::MoveTo(move_x, move_y))?;

    Ok(())
}

/// Delete character at left directed offset from cursor on current row if possible
fn delete(display: &mut Display, offset: i32) -> Result<()> {
    let (cur_col, cur_row) = cursor::position()?;
    
    let pos = cur_col as i32 - offset;
    if pos < 0 && offset != 0 {  //do not splice on delete
        splice_line(display)?;
        
        return Ok(());
    }
    
    if (cur_row as usize) < display.lines.len() {
        let cur_line = &mut display.lines[cur_row as usize];
        let pos = pos as usize;
        if pos < cur_line.len() {
            cur_line.remove(pos);
    
            if offset > 0 {
                move_cursor(display, 0, -(offset as i32))?;
            }
        }
    }

    Ok(())
}

/// Delete character starting at cursor until end of line
fn kill(display: &mut Display) -> Result<()> {
    let (cur_col, cur_row) = cursor_pos_usize();

    if cur_row < display.lines.len() {
        let cur_line = display.lines[cur_row].clone();

        if cur_col == 0 && cur_line.is_empty() {
            splice_line(display)?;
            return Ok(());
        }
    
        if cur_col < cur_line.len() {
            let new_line = cur_line.chars().take(cur_col).collect();
            display.lines[cur_row] = new_line;
        }
    }
    
    Ok(())
}

fn insert(display: &mut Display, string: &str) -> Result<()> {
    let (cur_col, cur_row) = cursor_pos_usize();
    let cur_line = &mut display.lines[cur_row];
    
    if cur_col < cur_line.len() { cur_line.insert_str(cur_col, string); }
    else { cur_line.push_str(string); }
    move_cursor(display, 0, string.len() as i32)?;

    Ok(())
}

/// split current line into two at cursor 
fn split_line(display: &mut Display) -> Result<()> {
    let (cur_col, cur_row) = cursor_pos_usize();

    let cur_line = display.lines[cur_row].clone();
    let first = cur_line.chars().take(cur_col).collect();
    let second = cur_line.chars().skip(cur_col).collect();

    display.lines[cur_row] = first;

    if cur_row < display.lines.len() {
        display.lines.insert(cur_row + 1, second);
    }
    else {
        display.lines.push(second);
    }
    move_cursor(display, 1, 0)?;
    move_cursor_abs(display, None, Some(0))?;

    Ok(())
}

fn splice_line(display: &mut Display) -> Result<()> {
    let (_cur_col, cur_row) = cursor_pos_usize();

    if cur_row >= display.lines.len() {
        return Ok(());
    }

    let cur_line = display.lines[cur_row].clone();
    if cur_row > 0 && !cur_line.is_empty() {
        display.lines.remove(cur_row);
        let prev_line = &mut display.lines[cur_row - 1];
        prev_line.push_str(&cur_line);
        move_cursor(display, -1, 0)?;
    }
    else if cur_line.is_empty() {
        display.lines.remove(cur_row);
    }

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
