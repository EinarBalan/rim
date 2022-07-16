use std::{
    io::{Stdout, Write}, 
    collections::{LinkedList, linked_list::CursorMut}, 
    process
};

use crossterm::{
    execute, queue,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, DisableLineWrap},
    cursor,
    style::*,
    Result,
};

use super::{list, control};

pub struct Display  {
    pub stdout: Stdout,
    pub lines: LinkedList<String>,
}

impl Display {
    pub fn new(mut stdout: Stdout, content: &String) -> Display {
        let lines = content.lines().collect();
        let mut lines = list::from(&lines);
        if let Err(e) = Display::init(&mut stdout, &lines) {
            eprintln!("Error while initializing display: {}", e);
            process::exit(1);
        }

        Display { stdout, lines }
    }
    
    fn init(stdout: &mut Stdout, lines: &LinkedList<String>) -> Result<()> {
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

    pub fn refresh(&mut self) -> Result<()> {
        let lines = &self.lines;
        for line in lines {
            queue!(
                self.stdout,
                Print(&line),
                cursor::MoveToNextLine(1),
            )?;
        }
        self.stdout.flush()?;

        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        execute!(self. stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}