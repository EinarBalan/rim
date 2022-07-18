use std::{
    io::{Stdout, Write}, 
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

use super::{
    vector, 
    config::Config,
};

pub struct Display  {
    pub stdout: Stdout,
    pub lines: Vec<String>,
    pub file_name: String,
}

impl Display {
    pub fn new(stdout: Stdout, content: &str, config: &Config) -> Display {
        let lines = content.lines().collect();
        let lines = vector::from(lines);

        Display { stdout, lines, file_name: config.file_name.clone()}
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
            Clear(ClearType::All),
        )?;

        self.print_lines()?;

        execute!(self.stdout, cursor::RestorePosition)?;

        Ok(())
    }

    /// prints all lines to stdout
    fn print_lines(&mut self) -> Result<()> {
        for line in &self.lines {
            queue!(
                self.stdout,
                Print(line),
                cursor::MoveToNextLine(1),
            )?;
        }
        self.stdout.flush()?;

        Ok(())
    }
}

/// return terminal to normal state on drop
impl Drop for Display {
    fn drop(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).expect("Error on leaving alternate screen.");
        disable_raw_mode().expect("Error on disabling raw mode.");
    }
}

/// Returns the cursor position (col, row) as usize
pub fn cursor_pos_usize() -> Result<(usize, usize)> {
    let (x, y) = cursor::position()?;

    Ok((x as usize, y as usize))
}

#[allow(dead_code)]
/// Returns the terminal size (columns, rows) as usize
pub fn terminal_usize() -> Result<(usize, usize)> {
    let (col, row) = terminal::size()?;

    Ok((col as usize, row as usize))
}