// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...

use std::env;
/*
use std::error::Error;
#[derive(Debug, Default)]
struct Options {
    ascii:       bool,
    c_flag:      bool,
    d_flag:      bool,
    ignore_case: bool,
    r_flag:      bool,
    init_search: Option<String>,
    max_x:       Option<i32>,
    max_y:       Option<i32>,
    files:       Vec<String>,
}

impl Options {
    fn parse() -> Result<Self, String> {
        let mut opts = Self::default();
        let mut args = env::args_os().skip(1); // skip program name

        while let Some(arg_os) = args.next() {
            // Convert only if valid UTF-8; otherwise bail out early with context.
            let arg = arg_os
                .into_string()
                .map_err(|_| "argument is not valid UTF-8".to_string())?;

            match arg.as_str() {
                a if a.starts_with('-') => {
                    let tail = &a[1..];

                    // If the whole tail parses as an int we take it as `max_x`.
                    if opts.max_x.is_none() {
                        if let Ok(n) = tail.parse::<i32>() {
                            opts.max_x = Some(n);
                            continue;
                        }
                    }

                    // Otherwise each char is a single-letter switch.
                    for ch in tail.chars() {
                        match ch {
                            'a' => opts.ascii = true,
                            'c' => opts.c_flag = true,
                            'd' => opts.d_flag = true,
                            'i' => opts.ignore_case = true,
                            'r' => opts.r_flag = true,
                            other => {
                                return Err(format!("unknown flag '-{other}'"));
                            }
                        }
                    }
                }

                a if a.starts_with("+/") => {
                    opts.init_search = Some(a[2..].to_string());
                }

                a if a.starts_with('+') => {
                    let n = &a[1..];
                    match n.parse::<i32>() {
                        Ok(num) => opts.max_y = Some(num),
                        Err(_) => return Err(format!("invalid number after '+': {}", n)),
                    }
                }

                // Positional argument → treat as file.
                file => opts.files.push(file.to_string()),
            }
        }
        Ok(opts)
    }
}
 
fn main() -> Result<(), Box<dyn Error>>{
    let opts = Options::parse()?;
    println!("{:#?}", opts);
    Ok(())
}
*/
 fn main()  {

    let mut ascii_flag : bool = false;
    let mut c_flag : bool = false;
    let mut d_flag : bool = false;
    let mut ignore_case : bool = false;
    let mut r_flag : bool = false;
    let mut init_search : bool  = false;
    let mut ssearch = String::new();
    let mut files: Vec<String> = Vec::new();
    let mut mymaxx: i32 = 0;
    let mut mymaxy: i32 = 0;

    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        let a = arg.clone();
        dbg!(a);
        if arg.starts_with('-') {
            let ascidir = arg[1..].to_string();
            println!("ascidir: {}", ascidir);    
            mymaxx = match ascidir.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };

            for c in ascidir.chars() {
                if c == 'a' {
                    ascii_flag = true;
                } else if c == 'c' {
                    c_flag = true;
                } else if c == 'd' {
                    d_flag = true;
                } else if c == 'i' {
                    ignore_case = true;
                } else if c == 'r' {
                    r_flag = true;
                }
            }
        } else if arg.starts_with("+/") {
            init_search = true;
            ssearch = arg[2..].to_string();
        } else if arg.starts_with("+") {
            let offset_for_char = arg[1..].to_string(); 
            println!("offset_for_char: {}", offset_for_char);    
            mymaxy = match offset_for_char.trim().parse() {
                Ok(num) => num,
                Err(_) => 0
            };
        } else {
            // everything I put in files!
            files.push(arg.to_string());
        }


    }
   println!("ascii_flag: {}", ascii_flag);
   println!("c_flag: {}", c_flag);
   println!("d_flag: {}", d_flag);
   println!("ignore_case: {}", ignore_case);
   println!("r_flag: {}", r_flag);
   println!("init_search: {}", init_search);
   println!("ssearch: {}", ssearch);
   println!("mymaxx: {}", mymaxx);
   println!("mymaxy: {}", mymaxy);
   println!("init_search: {}", init_search);
   println!("files: {:?}", files);
}