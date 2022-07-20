use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    queue, Result,
};
use std::{cmp, fs, io::Write, time::Duration};

use super::{
    display::{self, Display},
    buf,
};
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
    End,
}

pub struct Editor {
    display: Display,
    copied: Option<Vec<String>>
}

impl Editor {
    pub fn new(display: Display) -> Editor {
        Editor { display, copied: None }
    }

    pub fn event_loop(&mut self) -> Result<()> {
        loop {
            // listen for key
            if poll(Duration::from_millis(1000))? {
                if let Event::Key(key_event) = read()? {
                    // exit program on escape or Ctrl-X
                    if key_event.code == KeyCode::Esc ||
                        (key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('x'))
                    {
                        break;
                    } else {
                        self.handle_key_event(key_event)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Result<()> {
        let result = match event {
            // standard controls (no modifiers applied)
            KeyEvent { modifiers: KeyModifiers::NONE, code } => {
                match code {
                    KeyCode::Left => self.move_cursor(Direction::Left),
                    KeyCode::Right => self.move_cursor(Direction::Right),
                    KeyCode::Up => self.move_cursor(Direction::Up),
                    KeyCode::Down => self.move_cursor(Direction::Down),
                    KeyCode::Enter => {
                        self.split_line()?;
                        self.display.refresh()
                    }
                    KeyCode::Backspace => {
                        self.delete(1)?;
                        self.display.refresh()
                    }
                    KeyCode::Char(c) => {
                        let string = String::from(c);
                        self.insert(&string)?;
                        self.display.refresh()
                    }
                    KeyCode::Tab => {
                        self.insert("    ")?;
                        self.display.refresh()
                    }
                    _ => Ok(()),
                }
            }

            // CTRL modifer
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
                match code {
                    KeyCode::Char('b') => self.move_cursor(Direction::Left),
                    KeyCode::Char('f') => self.move_cursor(Direction::Right),
                    KeyCode::Char('p') => self.move_cursor(Direction::Up),
                    KeyCode::Char('n') => self.move_cursor(Direction::Down),
                    KeyCode::Char('a') => self.move_cursor(Direction::Start),
                    KeyCode::Char('e') => self.move_cursor(Direction::End),
                    KeyCode::Char('d') => {
                        self.delete(0)?;
                        self.display.refresh()
                    }
                    KeyCode::Char('k') => {
                        self.kill()?;
                        self.display.refresh()
                    }
                    KeyCode::Char('s') => {
                        self.save()
                    }
                    KeyCode::Char('y') => {
                        self.paste()?;
                        self.display.refresh()
                    }
                    _ => Ok(()),
                }
            }

            // Shift modifier
            KeyEvent { modifiers: KeyModifiers::SHIFT, code: KeyCode::Char(c) } => {
                self.insert(&c.to_uppercase().to_string())?;
                self.display.refresh()
            }

            _ => Ok(()),
        };
        self.display.stdout.flush()?;

        result
    }

    /// Move cursor y rows and x columns if possible.
    /// Will not move if outside the bounds of the text.
    fn move_cursor(&mut self, dir: Direction) -> Result<()> {
        let lines = &mut self.display.lines;
        let stdout = &mut self.display.stdout;

        let (mut col, mut row) = display::cursor_pos_usize()?;

        if let Some(line) = lines.get(row) {
            match dir {
                Direction::Up => if row != 0 { row -= 1; },
                Direction::Down => row += 1,
                Direction::Left => if col != 0 { col -= 1; },
                Direction::Right => col += 1,
                Direction::Start => col = 0,
                Direction::End => col = line.len(),
            }

            // don't move to new row if it is outside bounds
            let num_rows = lines.len();
            if row >= num_rows {
                return Ok(());
            }

            // force column to always be in bounds
            let num_cols = lines[row as usize].len();
            col = cmp::min(col, num_cols);

            queue!(stdout, cursor::MoveTo(col as u16, row as u16))?;
        }

        Ok(())
    }

    /// Delete character at left directed offset from cursor on current row if possible
    fn delete(&mut self, offset: i32) -> Result<()> {
        let (cur_col, cur_row) = display::cursor_pos_usize()?;

        let pos = cur_col as i32 - offset;
        if pos < 0 && offset != 0 {
            //do not splice on delete
            self.splice_up()?;

            return Ok(());
        }

        let cur_line = &mut self.display.lines[cur_row];
        let pos = pos as usize;
        if pos < cur_line.len() {
            cur_line.remove(pos);

            for _ in 0..offset {
                self.move_cursor(Direction::Left)?;
            }
        }

        Ok(())
    }

    /// Delete character starting at cursor until end of line
    fn kill(&mut self) -> Result<()> {
        let (cur_col, cur_row) = display::cursor_pos_usize()?;

        let cur_line = &mut self.display.lines[cur_row];

        if cur_col == 0 && cur_line.is_empty() {
            self.splice_up()?;
            return Ok(());
        }

        if cur_col < cur_line.len() {
            // kill to end and copy
            // let new_line = cur_line.iter().take(cur_col).collect();
            // let killed = cur_line.iter().skip(cur_col).collect();
            let killed = cur_line.drain(cur_col..).collect();
            self.copied = Some(vec![killed]);
        } else {
            self.splice_down()?;
        }

        Ok(())
    }

    /// Paste copied text at cursor
    fn paste(&mut self) -> Result<()> {
        let copied = self.copied.clone();
        if let Some(copied) = copied {
            for line in copied {
                self.insert(&line)?;
            }
        }

        Ok(())
    }

    /// Insert string at cursor
    fn insert(&mut self, string: &str) -> Result<()> {
        let (cur_col, cur_row) = display::cursor_pos_usize()?;
        let cur_line = &mut self.display.lines[cur_row];

        if cur_col < cur_line.len() {
            cur_line.insert_many(cur_col, string.chars());
        } else {
            buf::push_owned(cur_line, string.chars());
        }
        for _ in 0..string.len() {
            self.move_cursor(Direction::Right)?;
        }

        Ok(())
    }

    /// Split current line into two at cursor
    fn split_line(&mut self) -> Result<()> {
        let (cur_col, cur_row) = display::cursor_pos_usize()?;

        let cur_line = &mut self.display.lines[cur_row];
        let new_line = cur_line.drain(cur_col..).collect();
        // let first = cur_line.iter().take(cur_col).collect();
        // let second = cur_line.iter().skip(cur_col).collect();

        // self.display.lines[cur_row] = cur_line;

        if cur_row < self.display.lines.len() {
            self.display.lines.insert(cur_row + 1, new_line);
        } else {
            self.display.lines.push_back(new_line);
        }
        self.move_cursor(Direction::Down)?;
        self.move_cursor(Direction::Start)?;

        Ok(())
    }


    /// Merge current line with line above
    fn splice_up(&mut self) -> Result<()> {
        let (_cur_col, cur_row) = display::cursor_pos_usize()?;

        let num_rows = self.display.lines.len();

        if cur_row < num_rows - 1 {   
            let cur_line = self.display.lines[cur_row].clone();

            if cur_line.is_empty(){
                self.display.lines.remove(cur_row);
            } else if cur_row > 0 {
                self.move_cursor(Direction::Up)?;
                self.move_cursor(Direction::End)?;
    
                self.display.lines.remove(cur_row);
    
                let prev_line = &mut self.display.lines[cur_row - 1];
                buf::push_ref(prev_line, cur_line.iter());
            } 
        } else {
            self.move_cursor(Direction::Up)?;
        }

        Ok(())
    }

    /// Merge current line with line below
    fn splice_down(&mut self) -> Result<()> {
        let (_cur_col, cur_row) = display::cursor_pos_usize()?;

        let num_rows = self.display.lines.len();

        if cur_row < num_rows - 1 {
            let next_line = self.display.lines[cur_row + 1].clone();
            self.display.lines.remove(cur_row + 1);

            let cur_line = &mut self.display.lines[cur_row];
            buf::push_ref(cur_line, next_line.iter());        
        }

        Ok(())
    }

    /// Save edits to current file
    fn save(&mut self) -> Result<()> {
        let lines = &self.display.lines;
        let file_name = &mut self.display.file_name.clone();
        let mut content = String::new();
        for line in lines {
            content.push_str(&format!("{}\n", buf::to_string(line))[..]);
        }

        fs::write(file_name, content)?;

        Ok(())
    }
}

