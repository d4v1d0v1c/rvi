use std::env;

#[derive(Debug, Default)]
pub struct Options {
    ascii: bool,
    c_flag: bool,
    d_flag: bool,
    ignore_case: bool,
    r_flag: bool,
    init_search: bool,
    ssearch: String,
    mymaxx: i32,
    mymaxy: i32,
    files: Vec<String>,
}

impl Options {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();
        let mut opts = Self::default();
        for arg in &args[1..] {
            if arg.starts_with('-') {
                let ascidir = arg.strip_prefix('-').unwrap_or_default().to_string();
                // let ascidir = arg[1..].to_string();
                opts.mymaxx = ascidir.trim().parse().unwrap_or_default();

                for c in ascidir.chars() {
                    if c == 'a' {
                        opts.ascii = true;
                    } else if c == 'c' {
                        opts.c_flag = true;
                    } else if c == 'd' {
                        opts.d_flag = true;
                    } else if c == 'i' {
                        opts.ignore_case = true;
                    } else if c == 'r' {
                        opts.r_flag = true;
                    }
                }
            } else if arg.starts_with("+/") {
                opts.init_search = true;
                //flags.ssearch = arg[2..].to_string();
                opts.ssearch = arg.strip_prefix("+/").unwrap_or_default().to_string();
            } else if arg.starts_with("+") {
                //let offset_for_char = arg[1..].to_string();
                let offset_for_char = arg.strip_prefix('+').unwrap_or_default().to_string();
                opts.mymaxy = offset_for_char.trim().parse().unwrap_or_default();
            } else {
                // everything I put in files!
                opts.files.push(arg.to_string());
            }
        }
        println!("flags: {opts:?}");
        Ok(opts)
    }
}
