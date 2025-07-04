use clap::{
    crate_name, crate_version, value_parser, Arg, ArgAction, ArgGroup, ColorChoice, Command,
};

use once_cell::sync::Lazy;
use std::env;
use std::path::{Path, PathBuf};

static VERSION: Lazy<String> = Lazy::new(|| {
    #[cfg(feature = "bugreport")]
    let git_version = bugreport::git_version!(fallback = "");
    #[cfg(not(feature = "bugreport"))]
    let git_version = "";

    if git_version.is_empty() {
        crate_version!().to_string()
    } else {
        format!("{} ({})", crate_version!(), git_version)
    }
});

pub fn build_app(interactive_output: bool) -> Command {
    let color_when = if interactive_output && !crate::app::env_no_color() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let mut app = Command::new(crate_name!())
        .version(VERSION.as_str())
        .color(color_when)
        .hide_possible_values(true)
        .args_conflicts_with_subcommands(true)
        .allow_external_subcommands(true)
        .disable_help_subcommand(true)
        .max_term_width(100)
        .about("A bmore clone.")
        .long_about("A bmore clone.")
        .arg(Arg::new("FILE")
            .help("File(s) to print / concatenate. Use '-' for standard input.")
            .long_help(
            "File(s) to print / concatenate. Use a dash ('-') or no argument at all \
            to read from standard input.",
        )
        .num_args(1..)
        .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("ascii")
            .long("ascii")
            .action(ArgAction::SetTrue)
            .short('a')
            .help("Show ascii printable chars")
            .long_help(
                "Show ascii printable chars in the right part of screen."
            ),
        )
        .arg(
            Arg::new("c_flag")
            .long("c_flag")
            .action(ArgAction::SetTrue)
            .short('c')
            .help("Clear the screen.")
            .long_help(
                "Clear the screen."
            )
        )
        .arg(Arg::new("d_flag")
            .long("d_flag")
            .action(ArgAction::SetTrue)
            .short('d')
            .help("Interactive mode")
            .long_help(
                "Interactive mode ?"
            )
        )
        .arg(Arg::new("ignore_case")
            .long("ingore_case")
            .action(ArgAction::SetTrue)
            .short('i')
            .help("Ignore letter case")
            .long_help("Ignore letter case during search")
        )
        .arg(Arg::new("r_flag")
            .long("r_flag")
            .action(ArgAction::SetTrue)
            .short('r')
            .help("Print ascii values")
            .long("Print ascii values otherwise print .")
        )
        .arg(Arg::new("range")
            .long("range")
            .action(ArgAction::Append)
            .value_name("X:Y")
            .short('r')
            .help("Set maxx:maxy")
            .long_help("Set maxx & maxy values, explain in details")
        )
        .arg(Arg::new("diagnostic")
            .long("diagnostic")
            .alias("diagnostics")
            .action(ArgAction::SetTrue)
            .hide_short_help(true)
            .help("Show diagnostic information for bug reports."),
        )
        .arg(Arg::new("acknowledgements")
            .long("acknowledgements")
            .action(ArgAction::SetTrue)
            .hide_short_help(true)
            .help("Show acknowledgements."),
        )
        .arg(Arg::new("set-terminal-title")
            .long("set-terminal-title")
            .action(ArgAction::SetTrue)
            .hide_short_help(true)
            .help("Sets terminal title to filenames when using a pager."),
        );
        app
}
