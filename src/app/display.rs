use std::{
    io::{Stdout, Write}, 
    cmp, 
};
use crossterm::{
    execute, queue,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen,
        enable_raw_mode, disable_raw_mode,
        DisableLineWrap,
        Clear, ClearType, self, 
    },
    cursor,
    style::*,
    Result,
};
use gapbuf::GapBuffer;
use super::{
    buf, 
    config::Config,
};

pub struct Display  {
    pub stdout: Stdout,
    pub lines: GapBuffer<GapBuffer<char>>,
    pub first_row: usize,
    pub first_col: usize,
    pub file_name: String,
}

impl Display {
    pub fn new(stdout: Stdout, content: &str, config: &Config) -> Display {
        let lines = buf::from_string(content);

        Display { 
            stdout, 
            lines, 
            first_row: 0,
            first_col: 0,
            file_name: config.file_name.clone()}
    }
    
    pub fn show(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, 
            EnterAlternateScreen,
            DisableLineWrap,
            cursor::MoveTo(0, 0),
        )?;

        self.print_lines()?;

        queue!(self.stdout, cursor::MoveTo(0, 0))?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        execute!(
            self.stdout, 
            cursor::SavePosition,
            cursor::MoveTo(0, 0),
            cursor::Hide,
            Clear(ClearType::All),
        )?;

        self.print_lines()?;

        execute!(
            self.stdout, 
            cursor::RestorePosition, 
            cursor::Show
        )?;

        Ok(())
    }

    fn add_new_line(&mut self) {
        let last = self.lines.len() - 1;
        let last_line = &self.lines[last];
        if !last_line.is_empty() {
            self.lines.push_back(GapBuffer::new());
        }
    }

    pub fn refresh_line(&mut self) -> Result<()> {
        self.add_new_line();

        let (_, cur_term_row) = cursor::position()?;
        let (_cur_col, cur_row) = self.cursor_pos_diplaced()?;
        if let Some(cur_line) = &self.lines.get(cur_row) {
            execute!(
                self.stdout, 
                cursor::SavePosition,
                cursor::MoveTo(0, cur_term_row),
                Clear(ClearType::CurrentLine),
                Print(buf::to_string(cur_line)),
                cursor::RestorePosition,
            )?;
        }
        Ok(())
    }

    /// prints all lines to stdout
    fn print_lines(&mut self) -> Result<()> {
        self.add_new_line();

        let (_cols, rows) = terminal_usize()?;
        let num_lines = self.lines.len();
        let last = cmp::min(num_lines, rows);

        let mut screen_lines = self.lines.range(self.first_row..); 
        if self.lines.len() > self.first_row + last {
            screen_lines = self.lines.range(self.first_row..(self.first_row + last)); 
        }

        for line in &screen_lines {
            // handle lines shorter than first col
            let mut range = String::new();
            let (num_cols, first_col) = (line.len() as i32, self.first_col as i32);
            if num_cols - first_col > 0 { 
                range = line.range(self.first_col..).iter().collect(); 
            } 
            queue!(
                self.stdout,
                Print(range),
                cursor::MoveToNextLine(1),
            )?;
        }
        self.stdout.flush()?;

        Ok(())
    }

    pub fn move_down(&mut self) {
        let num_lines = self.lines.len() as i32;
        let (_cols, rows) = terminal_usize()
            .expect("Something went wrong finding size of terminal");
        let rows = rows as i32;
        if (self.first_row as i32) < (num_lines - rows - 1) {
            self.first_row += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.first_row > 0 {
            self.first_row -= 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.first_col > 0 {
            self.first_col -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn move_right(&mut self) {
        self.first_col += 1;
    }

    pub fn cursor_pos_diplaced(&self) -> Result<(usize, usize)> {
        let (cur_col, cur_row) = cursor_pos_usize()?;
        let pos = (cur_col + self.first_col, cur_row + self.first_row);
    
        Ok(pos)
    }
    
}

/// return terminal to normal state on drop
impl Drop for Display {
    fn drop(&mut self) {
        execute!(self.stdout, cursor::Show, LeaveAlternateScreen, ).expect("Error on leaving alternate screen.");
        disable_raw_mode().expect("Error on disabling raw mode.");
    }
}

/// Returns the cursor position in GapBuffer (col, row) as usize
pub fn cursor_pos_usize() -> Result<(usize, usize)> {
    let (x, y) = cursor::position()?;
    let pos = (x as usize, y as usize);

    Ok(pos)
}


/// Returns the terminal size (columns, rows) as usize
pub fn terminal_usize() -> Result<(usize, usize)> {
    let (col, row) = terminal::size()?;

    Ok((col as usize, row as usize))
}