use core::fmt;
use std::fmt::Display;

use clap::{arg, Command};
use url::Url;

pub fn basic_command(name: &str) -> Command {
    return Command::new(name.to_string())
        .arg_required_else_help(true)
        .arg(arg!(<url>).required(true));
}

struct Error {}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error").finish()
    }
}

type Result<T> = ::core::result::Result<T, Error>;
