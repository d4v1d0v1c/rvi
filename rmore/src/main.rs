// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...

mod bytereader;

use std::fs::File;
use std::io::{self, ErrorKind, Read};
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
    bytepos: u16,
    screen_home: u16,
    should_quit: bool,
    prompt: u16,
    to_print: u16,
    do_header: u16,
    corr : u16,
    dup_print_flag: bool,
    precount : u16,
    z_line : u16,
    d_line : u16,
    r_line : u16,
}


impl BMore {
    pub fn printline(&mut self, buf :&Vec<u8>, num : u16) {
        print!("{:#08x}",self.bytepos);
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
        }
        print!("\r\n");
    }
        
    pub fn printout(&mut self, reader:&mut ByteReader<Box<File>>, lns : u16) -> u16 {
        let mut buffer1: Vec<u8> = Vec::new();
        let mut lns = lns;

        if self.flags.c_flag {
            Terminal::clearscreen();
        }

        if self.do_header > 0 {
            std::format!("::::::::::::::\n{}\n::::::::::::::\n", self.name);
            Terminal::cleartoeol();
            std::format!("::::::::::::::\r\n{}\r\n::::::::::::::\r\n", self.name);
        }
        self.do_header = 0;
        self.corr = 2;
        if self.corr > 0 && 
           lns > self.maxy - 2 {
            lns -= self.corr;
        } 
        self.corr = 0;
        let mut inum = 0;
        while inum < self.out_len {
            let byte = reader.next_byte().unwrap().expect("msg");
            buffer1.push(byte);
            inum += 1;
        }
        self.printline(&buffer1, inum);
        lns -= 1;
        0
    }

    pub fn emsg(&mut self, s :&str) {
        Terminal::cleartoeol();
        Terminal::highlight();
        std::format!("{}", s);
        Terminal::normal();
        // fflush(stdout);
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
        dbg!("terminal: w: {self.maxx}, h: {self.maxy");

        if self.mymaxy > 0 {
            dbg!("Overwrite maxy");
            self.maxy = self.mymaxy;
        }

        if self.mymaxx > 0 {
            dbg!("Overwrite out_len");
            self.out_len = self.mymaxx;
        }
    }

    pub fn nextchar<R: Read>(&mut self, reader: &mut ByteReader<Box<File>>) -> io::Result<Option<u8>> {
        if let Some(b) = reader.next_byte().unwrap() {
            Ok(Some(b))
        } else {
            Ok(None)
        }
    }

    pub fn run(&mut self) {
        self.set_size();
        let name = self.flags.files.pop().expect("Files are empty");
        let current_file = open_file(&name).unwrap();
        // reader: 
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
                std::format!("--More--");

                if self.flags.d_flag {
                    std::format!("[Press space to continue, 'q' to quit]");
                }
                Terminal::normal();
                //fflush(stdout);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            self.prompt = 1;
            std::format!("\r");

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
            Key::Ctrl('q' | 'Q') | Key::Char('q') => {
                Terminal::cleartoeol();
                Terminal::reset_tty();
                self.should_quit = true;
            }
            /*  Display next k lines of text [current screen size] */
            Key::Ctrl(' ') => {
                self.dup_print_flag = true;
                if self.precount > 0 {
                    self.to_print = self.precount;
                } else {
                    self.to_print = self.maxx;
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
        self.scroll();
        Ok(())
    }
    fn scroll(&mut self) {
        dbg!("scroll");
    }
    fn move_cursor(&mut self, _key: Key) {
        dbg!("move_cursor");
    }
    #[must_use] pub fn default() -> Self {
        Self {
            flags: Options::parse().unwrap_or_default(),
            terminal: Terminal::default().expect("Failed to init terminal"),
            name : "".to_string(),
            should_quit: false,
            maxx: 80,
            maxy: 25,
            mymaxx: 0,
            mymaxy: 0,
            out_len: 0,
            bytepos: 0,
            screen_home: 0,
            prompt: 0,
            to_print: 0,
            do_header: 0,
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
