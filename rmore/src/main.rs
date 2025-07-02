// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...

mod bytereader;

use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::Path;
use std::process::exit;
use termion::event::Key;
use util::{COPYRIGHT, Options, Terminal, AnzAdd, USAGE};
pub use bytereader::ByteReader;

pub struct BMore {
    terminal: Terminal,
    flags: Options,
    name: String,
    maxx: u16,
    maxy: u16,
    mymaxx: u16,
    mymaxy: u16,
    out_len: u16,
    bytepos: i32,
    screen_home: i32,
    should_quit: bool,
    prompt: u16,
    to_print: u16,
    do_header: bool,
    corr : u16,
    dup_print_flag: bool,
    precount : u16,
    z_line : u16,
    d_line : u16,
    r_line : u16,
}


impl BMore {
    pub fn printline(&mut self, buf :&Vec<u8>, _num : u16) {
        print!("{:#08x}  ",self.bytepos);
        let mut print_pos = 0_usize;

        if !self.flags.ascii {
            for byte in buf {
                print!("{byte:02x} ");
                print_pos += 1;
            }
            while print_pos < self.out_len.into() {
               print!("   ");   // three spaces per slot
               print_pos += 1;
            }
            print!(" ");
        }

        for c in buf {
            self.bytepos += 1;
            let ch : char = if (32..=126).contains(c) {
                *c as char
            } else if self.flags.r_flag &&
                      (c & 0x80 != 0) &&
                      (160..=254).contains(c) {
                (c & 0x7F) as char
            } else  {
                '.'
            };
            print!("{ch}");
        }

        while print_pos < self.out_len.into() {
            self.bytepos+=1;
            print!(" ");
            print_pos += 1;
        }
        print!("\r\n");
    }

    pub fn printout(&mut self, reader:&mut ByteReader<Box<File>>, lns : u16) -> io::Result<()> {
        let mut lns = lns;
        if self.flags.c_flag {
            Terminal::clearscreen();
        }

        if self.do_header {
            Terminal::cleartoeol();
            print!("::::::::::::::\r\n{}\r\n::::::::::::::\r\n", self.name);
        self.do_header = false;
        self.corr = 2;
        }

        if self.corr > 0 && 
           lns > self.maxy - 2 {
           lns -= self.corr;
        }         
        self.corr = 0;

        loop {
            let mut num = 0;
            let mut buffer1: Vec<u8> = Vec::new();

            while num < self.out_len {
                let mut byte: u8 = 0; 
                let size = reader.nextchar(&mut byte)?;
                if size == 0 {
                    break;
                }            
                buffer1.push(byte);
                num += 1;
            }
            if num == 0 {
                return Ok(());
            }

            if buffer1.len() > 0 || self.bytepos == 0 {
                self.printline(&buffer1, num);
                lns -= 1;
            } else {
                Terminal::clearscreen();
                print!("*\r\n");
                self.bytepos += num as i32;
                lns -= 1;
            }
            if lns == 0 {
                self.screen_home = self.bytepos - ((self.maxy + 1) * self.out_len) as i32;
                if self.screen_home < 0 {
                    self.screen_home = 0;
                    return Err(io::Error::new(ErrorKind::NotFound, "file does not exist"));
                }
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn emsg(&mut self, s :&str) {
        Terminal::cleartoeol();
        Terminal::highlight();
        print!("{}", s);
        Terminal::normal();
        self.prompt = 0;
    }

    pub fn bmsearch(&mut self, _search : bool) {
        
        if self.flags.ssearch.len() == 0 {
            self.emsg("No previous regular expression");
            return;
        }        

    }
    pub fn set_size(&mut self) {
        self.maxx = self.terminal.size().width;
        self.maxy = self.terminal.size().height;
        // dbg!("terminal: w: {self.maxx}, h: {self.maxy");

        if self.mymaxy > 0 {
            dbg!("Overwrite maxy");
            self.maxy = self.mymaxy;
        }

        if self.mymaxx > 0 {
            dbg!("Overwrite out_len");
            self.out_len = self.mymaxx;
        }
    }
    pub fn open_file(&mut self) -> io::Result<File> {
        if self.flags.files.len() > 1 {
            self.do_header = true;
        }

        let path = if let Some(file_name) = self.flags.files.pop() {
            self.name = file_name.clone();
            let path_buf = Path::new(&file_name).to_path_buf();
            if !path_buf.exists() {
                return Err(io::Error::new(ErrorKind::NotFound, "file does not exist"));
            }
            path_buf
        } else {
            return Err(io::Error::new(ErrorKind::InvalidData, "file are empty"));
        };

        File::open(path)
    }

    pub fn run(&mut self) {
        self.set_size();        
        let current_file = self.open_file().unwrap();
        let mut reader = ByteReader::new(Box::new(current_file));        
        self.bytepos = 0;
        self.screen_home = 0;

        Terminal::initterm();
        Terminal::set_tty();

        if self.mymaxy > 0 {
            self.maxy = self.mymaxy;
        }

        self.z_line = self.maxy;
        self.d_line = self.maxy / 2;
        self.r_line = 1;        
        self.screen_home = self.bytepos;

        if self.flags.ascii {
            self.out_len = ((self.maxx - AnzAdd - 1) / 4) * 4;
        } else {
            self.out_len = ((self.maxx - AnzAdd - 1) / 16) * 4;
        }

        if self.mymaxx > 0 {
            self.out_len = self.mymaxx;
        }

        if self.flags.init_search {
            self.bmsearch(self.flags.init_search);
        }

        loop {
            self.to_print = 0;
            self.dup_print_flag = false;
            if self.prompt > 0 {
                Terminal::highlight();
                print!("--More--");

                if self.flags.d_flag {
                    print!("[Press space to continue, 'q' to quit]");
                }
                Terminal::normal();
                //fflush(stdout);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            self.prompt = 1;
            print!("\r");

            if self.should_quit {
                break;
            }
            if self.to_print > 0 {
                self.printout(&mut reader, self.to_print);
            }


        }
    }

    fn process_keypress(&mut self) -> std::result::Result<(), std::io::Error> {
        let pressed_key = Terminal::vgetc()?;
        match pressed_key {
            Key::Ctrl('Q') | Key::Char('q') => {
                Terminal::cleartoeol();
                Terminal::reset_tty();
                self.should_quit = true;
            }
            /*  Display next k lines of text [current screen size] */
            Key::Char(' ') => {
                self.dup_print_flag = true;
                if self.precount > 0 {
                    self.to_print = self.precount;
                } else {
                    self.to_print = self.maxy;
                }
            }
            Key::Ctrl('z') => {
             	self.dup_print_flag = true;
				if self.precount > 0 { 
                    self.z_line = self.precount;
                } 
                self.to_print = self.z_line;   
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
            Key::Char('n' | 'N') => {
                dbg!("Press /");
            }
            /**** help file ****/
            Key::Char('?' | 'h') => {
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
        Ok(())
    }
    fn move_cursor(&mut self, _key: Key) {
        dbg!("move_cursor");
    }
    #[must_use] pub fn default() -> Self {
        Self {
            flags: Options::parse().unwrap_or_default(),
            terminal: Terminal::default().expect("Failed to init terminal"),
            should_quit: false,
            name: String::new(),
            maxx: 80,
            maxy: 25,
            mymaxx: 0,
            mymaxy: 0,
            out_len: 0,
            bytepos: 0,
            screen_home: 0,
            prompt: 0,
            to_print: 0,
            do_header: false,
            corr : 0,
            dup_print_flag: false,
            precount: 0,
            z_line : 0,
            d_line : 0,
            r_line : 0,
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clearscreen();
    panic!("{}", e);
}

fn main() {
    println!("copyright: {COPYRIGHT:?}");
    BMore::default().run();
}
