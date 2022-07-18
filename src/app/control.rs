use crossterm::{
    queue,
    event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent},
    cursor,
    Result, 
};
use std::{
    io::Write,
    time::Duration, 
    cmp, fs,
};

use super::display::{self, Display};

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
    End,    
}

pub fn event_loop(display: &mut Display) -> Result<()> {
    loop {
        // listen for key
        if poll(Duration::from_millis(1000))? {
            if let Event::Key(key_event) = read()? { 
                // exit program on escape or Ctrl-X
                if key_event.code == KeyCode::Esc || 
                (key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('x')) { 
                    break 
                }
                else { handle_key_event(display, key_event)?; }
            }
        } 
    }

    Ok(())
}

fn handle_key_event(display: &mut Display, event: KeyEvent) -> Result<()> {
    match event {
        // standard controls (no modifiers applied)
        KeyEvent { modifiers: KeyModifiers::NONE, code } => {
            match code {
                KeyCode::Left => { move_cursor(display, Direction::Left) },
                KeyCode::Right => { move_cursor(display, Direction::Right) },
                KeyCode::Up => { move_cursor(display, Direction::Up) },
                KeyCode::Down => { move_cursor(display, Direction::Down) },
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
                _ => Ok(()),
            }
        },

        // CTRL modifer
        KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
            match code {
                KeyCode::Char('b') => { move_cursor(display, Direction::Left) },
                KeyCode::Char('f') => { move_cursor(display, Direction::Right) },
                KeyCode::Char('p') => { move_cursor(display, Direction::Up) },
                KeyCode::Char('n') => { move_cursor(display, Direction::Down) },
                KeyCode::Char('a') => { move_cursor(display, Direction::Start) },
                KeyCode::Char('e') => { move_cursor(display, Direction::End) },
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
                _ => Ok(()),
            }
        },

        // Shift modifier
        KeyEvent { modifiers: KeyModifiers::SHIFT, code: KeyCode::Char(c) } => {
            insert(display, &c.to_uppercase().to_string())?;
            display.refresh()
        },

        _ => Ok(())
    }

}

/// Move cursor y rows and x columns if possible.
/// Will not move if outside the bounds of the text.
fn move_cursor(display: &mut Display,  dir: Direction) -> Result<()> {
    let lines = &mut display.lines;
    let stdout = &mut display.stdout;

    let (mut col, mut row) = display::cursor_pos_usize()?;

    if let Some(line) = lines.get(row) {
        match dir {
            Direction::Up => { if row != 0 { row -= 1; } },
            Direction::Down => { row += 1; },
            Direction::Left => { if col != 0 {col -= 1; } },
            Direction::Right => { col += 1; },
            Direction::Start => { col = 0; },
            Direction::End => { col = line.len(); },
        }

        // don't move to new row if it is outside bounds
        let num_rows = lines.len();
        if row >= num_rows { return Ok(()); }

        // force column to always be in bounds
        let num_cols = lines[row as usize].len();
        col = cmp::min(col, num_cols);

        queue!(stdout, cursor::MoveTo(col as u16, row as u16))?;
        stdout.flush()?;
    }

    Ok(())
}

/// Delete character at left directed offset from cursor on current row if possible
fn delete(display: &mut Display, offset: i32) -> Result<()> {
    let (cur_col, cur_row) = display::cursor_pos_usize()?;
    
    let pos = cur_col as i32 - offset;
    if pos < 0 && offset != 0 {  //do not splice on delete
        splice_line(display)?;
        
        return Ok(());
    }
    
    if cur_row < display.lines.len() {
        let cur_line = &mut display.lines[cur_row];
        let pos = pos as usize;
        if pos < cur_line.len() {
            cur_line.remove(pos);
    
            for _ in 0..offset { move_cursor(display, Direction::Left)?; }
        }
    }

    Ok(())
}

/// Delete character starting at cursor until end of line
fn kill(display: &mut Display) -> Result<()> {
    let (cur_col, cur_row) = display::cursor_pos_usize()?;

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
    let (cur_col, cur_row) = display::cursor_pos_usize()?;
    let cur_line = &mut display.lines[cur_row];
    
    if cur_col < cur_line.len() { cur_line.insert_str(cur_col, string); }
    else { cur_line.push_str(string); }
    for _ in 0..string.len() { move_cursor(display, Direction::Right)?; }

    Ok(())
}

/// split current line into two at cursor 
fn split_line(display: &mut Display) -> Result<()> {
    let (cur_col, cur_row) = display::cursor_pos_usize()?;

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
    move_cursor(display, Direction::Down)?;
    move_cursor(display, Direction::Start)?;

    Ok(())
}

fn splice_line(display: &mut Display) -> Result<()> {
    let (_cur_col, cur_row) = display::cursor_pos_usize()?;

    let cur_line = display.lines[cur_row].clone();
    if cur_row >= display.lines.len() - 1 &&
        cur_line.is_empty() {
        move_cursor(display, Direction::Up)?;
    }

    if cur_row > 0 && !cur_line.is_empty() {
        move_cursor(display, Direction::Up)?;
        move_cursor(display, Direction::End)?;
        display.lines.remove(cur_row);
        let prev_line = &mut display.lines[cur_row - 1];
        prev_line.push_str(&cur_line);    }
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
