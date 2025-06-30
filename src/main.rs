// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...

#[derive(Debug, Default)]
struct Options {
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

use std::env;
fn main() {
    let mut flags = Options::default();
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        if arg.starts_with('-') {
            let ascidir = arg.strip_prefix('-').unwrap_or_default().to_string();
            // let ascidir = arg[1..].to_string();
            flags.mymaxx = ascidir.trim().parse().unwrap_or_default();

            for c in ascidir.chars() {
                if c == 'a' {
                    flags.ascii = true;
                } else if c == 'c' {
                    flags.c_flag = true;
                } else if c == 'd' {
                    flags.d_flag = true;
                } else if c == 'i' {
                    flags.ignore_case = true;
                } else if c == 'r' {
                    flags.r_flag = true;
                }
            }
        } else if arg.starts_with("+/") {
            flags.init_search = true;
            //flags.ssearch = arg[2..].to_string();
            flags.ssearch = arg.strip_prefix("+/").unwrap_or_default().to_string();
        } else if arg.starts_with("+") {
            //let offset_for_char = arg[1..].to_string();
            let offset_for_char = arg.strip_prefix('+').unwrap_or_default().to_string();
            flags.mymaxy = offset_for_char.trim().parse().unwrap_or_default();
        } else {
            // everything I put in files!
            flags.files.push(arg.to_string());
        }
    }
    println!("flags: {flags:?}");
}
