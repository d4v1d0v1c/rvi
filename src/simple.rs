use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

#[derive(Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    #[must_use] pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn initterm() {
    }

    pub fn set_tty() {
    }

    pub fn reset_tty() {
        print!("{}", termion::clear::All);
    }

    pub fn sig() {
    }

    pub fn doshell(_cmd: &str) {
    }

    pub fn highlight() {
    }

    pub fn normal() {
    }

    pub fn clearscreen() {
        print!("{}", termion::clear::All);
    }

    pub fn home() {
        print!("{}", termion::cursor::Goto(1, 1));
    }

    pub fn cleartoeol() {
        print!("{}", termion::clear::AfterCursor);
    }

    pub fn vgetc() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
