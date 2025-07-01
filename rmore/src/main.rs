// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...

use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::Path;
use std::process::exit;
use termion::event::Key;
use util::{COPYRIGHT, Options, Terminal, USAGE};

pub struct BMore {
    terminal: Terminal,
    flags: Options,
    maxx: u16,
    maxy: u16,
    mymaxx: u16,
    mymaxy: u16,
    out_len: u16,
    bytepos: u16,
    screen_home: u16,
    should_quit: bool,
}

fn die(e: std::io::Error) {
    Terminal::clearscreen();
    panic!("{}", e);
}

impl BMore {
    pub fn set_size(&mut self) {
        self.maxx = self.terminal.size().width;
        self.maxy = self.terminal.size().height;
        dbg!("terminal: w: {self.maxx}, h: {self.maxy");

        if self.mymaxy > 0 {
            dbg!("Overwrite maxy");
            self.maxy = self.mymaxy;
        }

        if self.mymaxx > 0 {
            dbg!("Overwrite out_len");
            self.out_len = self.mymaxx
        }
    }

    pub fn run(&mut self) {
        self.set_size();
        let file_name = self.flags.files.pop().expect("Files are empty");
        let _current_file = open_file(&file_name).unwrap();
        self.bytepos = 0;
        self.screen_home = 0;

        loop {
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&mut self) -> std::result::Result<(), std::io::Error> {
        let pressed_key = Terminal::vgetc()?;
        match pressed_key {
            Key::Ctrl('q') | Key::Ctrl('Q') | Key::Char('q') => {
                Terminal::cleartoeol();
                Terminal::reset_tty();
                dbg!("Got ctrl q");
                self.should_quit = true
            }
            /*  Display next k lines of text [current screen size] */
            Key::Ctrl(' ') | Key::Ctrl('z') => {
                dbg!("Press space or z");
            }
            /* Scroll k lines [current scroll size, initially 11]* */
            Key::Char('d') => {
                dbg!("Press d");
            }
            /*** REDRAW SCREEN ***/
            Key::Char('L') => {
                dbg!("Press L");
                Terminal::clearscreen();
            }
            /* Skip backwards k screenfuls of text [1] */
            Key::Char('b') => {
                dbg!("Press b");
            }
            /* Skip forward k screenfuls of bytes [1] */
            Key::Char('f') => {
                dbg!("Press f");
            }
            /* Skip forward k lines of bytes [1] */
            Key::Char('s') => {
                dbg!("Press s");
            }
            /**** Search String ****/
            Key::Char('/') => {
                dbg!("Press /");
            }
            /**** Search Next ****/
            Key::Char('n') | Key::Char('N') => {
                dbg!("Press /");
            }
            /**** help file ****/
            Key::Char('?') | Key::Char('h') => {
                println!("{USAGE}");
                exit(1);
            }

            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageDown
            | Key::PageUp
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }
    fn scroll(&mut self) {
        dbg!("scroll");
    }
    fn move_cursor(&mut self, _key: Key) {
        dbg!("move_cursor");
    }
    pub fn default() -> Self {
        Self {
            flags: Options::parse().unwrap_or_default(),
            terminal: Terminal::default().expect("Failed to init terminal"),
            should_quit: false,
            maxx: 80,
            maxy: 25,
            mymaxx: 0,
            mymaxy: 0,
            out_len: 0,
            bytepos: 0,
            screen_home: 0,
        }
    }
}

pub fn open_file(file_name: &str) -> io::Result<File> {
    let path = Path::new(file_name);
    if !path.exists() {
        return Err(io::Error::new(ErrorKind::NotFound, "file does not exist"));
    }
    File::open(path)
}

fn main() {
    println!("copyright: {COPYRIGHT:?}");
    BMore::default().run();
}
