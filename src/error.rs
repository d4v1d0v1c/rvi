use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] ::std::io::Error),
    #[error(transparent)]
    Fmt(#[from] ::std::fmt::Error),
    #[error(transparent)]
    ParseIntError(#[from] ::std::num::ParseIntError),
    #[error("unable to detect syntax for {0}")]
    UndetectedSyntax(String),
    #[error("unknown syntax: '{0}'")]
    UnknownSyntax(String),
    #[error("Unknown style '{0}'")]
    UnknownStyle(String),
    #[error("{0}")]
    Msg(String),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Msg(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}


pub type Result<T> = std::result::Result<T, Error>;

pub fn default_error_handler(error: &Error, output: &mut dyn Write) {
    use nu_ansi_term::Color::Red;

    match error {
        Error::Io(io_error) if io_error.kind() == ::std::io::ErrorKind::BrokenPipe => {
            ::std::process::exit(0);
        }
        _ => {
            writeln!(output, "{}: {}", Red.paint("[rmore error]"), error).ok();
        }
    };
}
