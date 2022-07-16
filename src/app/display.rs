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
        Clear, ClearType, 
    },
    cursor,
    style::*,
    Result,
};

use super::{vector, control};

pub struct Display  {
    pub stdout: Stdout,
    pub lines: Vec<String>,
}

impl Display {
    pub fn new(mut stdout: Stdout, content: &String) -> Display {
        let lines = content.lines().collect();
        let mut lines = vector::from(lines);
        if let Err(e) = Display::init(&mut stdout, &lines) {
            eprintln!("Error while initializing display: {}", e);
            process::exit(1);
        }

        Display { stdout, lines }
    }
    
    fn init(stdout: &mut Stdout, lines: &Vec<String>) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout, 
            EnterAlternateScreen,
            DisableLineWrap,
            cursor::MoveTo(0, 0),
        )?;

        for line in lines {
            queue!(
                stdout,
                Print(&line),
                cursor::MoveToNextLine(1),
            )?;
        }
        queue!(stdout, cursor::MoveTo(0, 0))?;
        stdout.flush()?;

        Ok(())
    }

    pub fn event_loop(&mut self) -> Result<()> {
        control::event_loop(self)?;

        Ok(())
    }

    pub fn refresh(mut stdout: &Stdout, lines: &Vec<String>) -> Result<()> {
        execute!(
            stdout, 
            cursor::SavePosition,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
        )?;

        for line in lines {
            queue!(
                stdout,
                Print(&line),
                cursor::MoveToNextLine(1),
            )?;
        }
        stdout.flush()?;

        execute!(stdout, cursor::RestorePosition)?;

        Ok(())
    }

    pub fn refresh_line (mut stdout: &Stdout, lines: &Vec<String>) -> Result<()> {

        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        execute!(self. stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}