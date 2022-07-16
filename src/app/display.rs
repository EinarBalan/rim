use std::{
    io::{Stdout, Write}, 
    collections::LinkedList, 
    process
};

use crossterm::{
    execute, queue,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    cursor,
    style::*,
    Result,
};

use super::{list, control};

pub struct Display {
    stdout: Stdout
}

impl Display {
    pub fn new(mut stdout: Stdout, content: &String) -> Display {
        let lines: Vec<&str> = content.lines().collect();
        let lines = list::from_vec(&lines);
        if let Err(e) = Display::init(&mut stdout, &lines) {
            eprintln!("Error while initializing display: {}", e);
            process::exit(1);
        }

        Display { stdout }
    }
    
    fn init(stdout: &mut Stdout, lines: &Vec<LinkedList<char>>) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout, 
            EnterAlternateScreen,
            cursor::MoveTo(0, 0),
        )?;

        for line in lines {
            queue!(
                stdout,
                Print(list::display(&line)),
                cursor::MoveToNextLine(1),
            )?;
        }
        queue!(stdout, cursor::MoveTo(0, 0))?;
        stdout.flush()?;

        Ok(())
    }

    pub fn event_loop(&self) -> Result<()> {
        control::event_loop(&self.stdout)?;

        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        execute!(self. stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}