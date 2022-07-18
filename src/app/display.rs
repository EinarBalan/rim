use std::{
    io::{Stdout, Write}, 
    process
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
    pub fn new(mut stdout: Stdout, content: &str, config: &Config) -> Display {
        let lines = content.lines().collect();
        let lines = vector::from(lines);
        if let Err(e) = Display::init(&mut stdout, &lines) {
            eprintln!("Error while initializing display: {}", e);
            process::exit(1);
        }

        Display { stdout, lines, file_name: config.file_name.clone()}
    }
    
    fn init(stdout: &mut Stdout, lines: &Vec<String>) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout, 
            EnterAlternateScreen,
            DisableLineWrap,
            cursor::MoveTo(0, 0),
        )?;

        Display::print_lines(stdout, lines)?;

        queue!(stdout, cursor::MoveTo(0, 0))?;
        stdout.flush()?;

        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        execute!(
            self.stdout, 
            cursor::SavePosition,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
        )?;

        Display::print_lines(&mut self.stdout, &self.lines)?;

        execute!(self.stdout, cursor::RestorePosition)?;

        Ok(())
    }

    /// prints a line at the position of the cursor, then moves cursor down
    fn queue_print_line(stdout: &mut Stdout, line: &str) -> Result<()> {
        queue!(
            stdout,
            Print(line),
            cursor::MoveToNextLine(1),
        )?;

        Ok(())
    }

    /// prints all lines to stdout
    fn print_lines(stdout: &mut Stdout, lines: &Vec<String>) -> Result<()> {
        for line in lines {
            Display::queue_print_line(stdout, line)?;
        }
        stdout.flush()?;

        Ok(())
    }

    #[allow(dead_code)]
    /// prints empty indicators starting from cursor until end of terminal
    /// ! Currently broken
    fn print_empty_indicators(stdout: &mut Stdout) -> Result<()> {
        let (_term_cols, term_rows) = terminal_usize()?;

        loop {
            let (_col, row) = cursor_pos_usize()?;

            if row == term_rows { break; }
            Display::queue_print_line(stdout, "~")?;
        }
        stdout.flush()?;

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

/// Returns the terminal size (columns, rows) as usize
pub fn terminal_usize() -> Result<(usize, usize)> {
    let (col, row) = terminal::size()?;

    Ok((col as usize, row as usize))
}