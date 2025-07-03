// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...

mod bytereader;

use std::fs::File;
use std::io::{self, ErrorKind};
use std::os::unix::process;
use std::path::Path;
use std::process::exit;
use termion::event::Key;
use util::{ANZADD, Options, Terminal,COPYRIGHT, USAGE};
pub use bytereader::ByteReader;

pub struct BMore {
    terminal: Terminal,
    flags: Options,
    name: String,
    maxx: i32,
    maxy: i32,
    mymaxx: i32,
    mymaxy: i32,
    out_len: i32,
    bytepos: i32,
    screen_home: i32,
    should_quit: bool,
    prompt: i32,
    to_print: i32,
    do_header: bool,
    corr : i32,
    dup_print_flag: bool,
    precount : i32,
    z_line : i32,
    d_line : i32,
    r_line : i32,
}


impl BMore {
    pub fn printline(&mut self, buf :&Vec<u8>, _num : i32) {
        print!("{:#08x}  ",self.bytepos);
        let mut print_pos : i32 = 0;

        if !self.flags.ascii {
            for byte in buf {
                print!("{byte:02x} ");
                print_pos += 1;
            }
            while print_pos < self.out_len {
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

        while print_pos < self.out_len {
            self.bytepos+=1;
            print!(" ");
            print_pos += 1;
        }
        print!("\r\n");
    }

    pub fn printout(&mut self, reader:&mut ByteReader<Box<File>>, lns : i32) -> io::Result<()> {
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
            let mut num : i32 = 0;
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
                self.bytepos += num;
                lns -= 1;
            }
            if lns == 0 {
                self.screen_home = self.bytepos - ((self.maxy + 1) * self.out_len);
                if self.screen_home < 0 {
                    self.screen_home = 0;
                    return Err(io::Error::new(ErrorKind::NotFound, "file does not exist"));
                }
                return Ok(());
            }
        }
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
        self.maxx = self.terminal.size().width as i32;
        self.maxy = self.terminal.size().height as i32;

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
        let current_file_result = self.open_file();
        let current_file = match current_file_result {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Problem opening the file: {e}");
                return;
            }
        };
        //let current_file = self.open_file().unwrap_or_else(|error| {
        //    panic!("Problem opening the file: {error:?}");
        //});

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
            self.out_len = (self.maxx - ANZADD / 4) * 4;
        } else {
            self.out_len = ((self.maxx - ANZADD - 1) / 16) * 4;
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
            if let Err(error) = self.process_keypress(&mut reader) {
                die(error);
            }
            self.prompt = 1;
            print!("\r");

            if self.should_quit {
                break;
            }
            if self.to_print > 0 {
                self.printout(&mut reader, self.to_print).unwrap_or_else(|err| println!("{:?}", err));
            }


        }
    }

    fn fseeko(&mut self, off: i32, reader: &mut ByteReader<Box<File>>) {

        let _ = reader.fseeko(off as u64);
    }

    fn process_keypress(&mut self, reader: &mut ByteReader<Box<File>>) -> std::result::Result<(), std::io::Error> {
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
                if self.precount > 0 {
                    self.d_line = self.precount;
                }
                self.to_print = self.d_line;
            }
            /*** REDRAW SCREEN ***/
            Key::Char('L') => {
                Terminal::clearscreen();
                self.to_print = self.maxy + 1;
                self.fseeko(self.screen_home, reader);
                self.bytepos = self.screen_home;
            }
            /* Skip backwards k screenfuls of text [1] */
            Key::Char('b') => {
                if self.precount < 1 {
                    self.precount = 1;
                }
                print!("...back {} page", self.precount);
     						if self.precount > 1 {
							      print!("s\r\n");
						    } else {
							      print!("\r\n");
						    }
						    self.screen_home -= (self.maxy + 1) * self.out_len;
						    if self.screen_home < 0 {
                    self.screen_home = 0;
						    } 
						    self.fseeko(self.screen_home, reader);
						    self.bytepos = self.screen_home;
						    self.to_print = self.maxy + 1;
            }
            /* Skip forward k screenfuls of bytes [1] */
            Key::Char('f') => {
                let mut count : i32 = 0;
                if self.precount < 1 {
                 self.precount = 1;
                }
                count = self.maxy * self.precount;
                print!("\r");
                Terminal::cleartoeol();
                print!("\n...skipping {} line", count);
                if count > 0 {
                    print!("s\r\n\r\n");
                } else {
                    print!("\r\n\r\n");                
                }
                self.screen_home += (count + self.maxy)*self.out_len;
                self.fseeko(self.screen_home, reader);
                self.bytepos = self.screen_home;
            }
            /* Skip forward k lines of bytes [1] */
            Key::Char('s') => {
                let mut count : i32 = 0;
                if self.precount < 1 {
                 self.precount = 1;
                }
                count = self.precount;
                print!("\r");
                Terminal::cleartoeol();
                print!("\n...skipping {} line", count);
                if count > 0 {
                    print!("s\r\n\r\n");
                } else {
                    print!("\r\n\r\n");                
                }
                self.screen_home += (count + self.maxy)*self.out_len;
                self.fseeko(self.screen_home, reader);
                self.bytepos = self.screen_home;
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

fn run() -> Result<bool> {
    BMore::default().run();
    return Ok(true);
}

fn main() {
    println!("copyright: {COPYRIGHT:?}");
    let result = run();
    match result {
        Err(error) => {
            let stderr = std::io::stderr();
            // default_error_handler(&error, &mut stderr.lock());
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}
