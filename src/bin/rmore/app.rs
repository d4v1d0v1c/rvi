use std::path::PathBuf;
use std::{env, path::Path};
use std::io::IsTerminal;

use crate::{
    clap_app,
    config::{get_args_from_env_vars},    
};
use clap::ArgMatches;
use rmore::area;
use crate::input::{new_file_input, new_stdin_input};

use rmore:: {
    area::ScreenArea,
    error::*,
    input::Input,
    config::Config,
};

pub fn env_no_color() -> bool {
    env::var_os("NO_COLOR").is_some_and(|x| !x.is_empty())
}

fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}


pub struct App {
    pub matches: ArgMatches,
    interactive_output: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let interactive_output = std::io::stdout().is_terminal();
        Ok(App {
            matches: Self::matches(interactive_output)?,
            interactive_output,
        })
    }

    fn matches(interactive_output: bool) -> Result<ArgMatches> {            
/*
        let args = if wild::args_os().any(|arg| arg == "--no-config") {
            let mut cli_args = wild::args_os();
            print!("cli_args: {:?}\n", cli_args);
        
            let mut args = get_args_from_env_vars();
            args.insert(0, cli_args.next().unwrap());
            cli_args.for_each(|a| args.push(a));
            args
        } else { 
            //TODO: add reading configuration file

        }
 */                
        let mut cli_args = wild::args_os();
        let mut args = get_args_from_env_vars();
        args.insert(0, cli_args.next().unwrap());
        cli_args.for_each(|a| args.push(a));

        Ok(clap_app::build_app(interactive_output).get_matches_from(args))
    }

    pub fn config(&self, inputs: &[Input]) -> Result<Config> {
        Ok(Config 
            { ascii: self.matches.get_flag("ascii"), 
                c_flag: self.matches.get_flag("c_flag"), 
                d_flag: self.matches.get_flag("d_flag"), 
                ignore_case: self.matches.get_flag("ignore_case"), 
                r_flag: self.matches.get_flag("r_flag"), 
                init_search: false, 
                ssearch: String::new(), 
                area: self.matches.get_one::<String>("range").map(|s| ScreenArea::from(s.as_str()).unwrap()).unwrap_or_default(),
            },
        )
    }

    pub fn inputs(&self) -> Result<Vec<Input>> { 

        let filenames: Option<Vec<&Path>> = self
            .matches
            .get_many::<PathBuf>("file-name")
            .map(|vs| vs.map(|p| p.as_path()).collect::<Vec<_>>());

        let files: Option<Vec<&Path>> = self
            .matches
            .get_many::<PathBuf>("FILE")
            .map(|vs| vs.map(|p| p.as_path()).collect::<Vec<_>>());
        if filenames.is_some() && files.is_some() && filenames.as_ref().map(|v| v.len()) != files.as_ref().map(|v| v.len()) {
            return Err("Must be one file name per input type.".into());
        }

        let mut filenames_or_none: Box<dyn Iterator<Item = Option<&Path>>> = match filenames {
            Some(filenames) => Box::new(filenames.into_iter().map(Some)),
            None => Box::new(std::iter::repeat(None)),
        };

        if files.is_none() {
            return Ok(vec![new_stdin_input(
                filenames_or_none.next().unwrap_or(None),
            )]);
        }
        let files_or_none: Box<dyn Iterator<Item = _>> = match files {
            Some(ref files) => Box::new(files.iter().map(|name| Some(*name))),
            None => Box::new(std::iter::repeat(None)),
        };

        let mut file_input = Vec::new();
        for (filepath, provided_name) in files_or_none.zip(filenames_or_none) {
            if let Some(filepath) = filepath {
                if filepath.to_str().unwrap_or_default() == "-" {
                    file_input.push(new_stdin_input(provided_name));
                } else {
                    file_input.push(new_file_input(filepath, provided_name));
                }
            }
        }
        Ok(file_input)
    }
}