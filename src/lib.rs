use core::fmt;
use std::{fmt::Display, io};

use async_trait::async_trait;
use clap::{arg, value_parser, Command};
use tokio::io::{AsyncRead, AsyncWrite};
use url::Url;

pub fn basic_command(name: &str) -> Command {
    return Command::new(name.to_string())
        .arg_required_else_help(true)
        .arg(arg!(<url>).value_parser(value_parser!(Url)).required(true));
}

pub trait IO: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static {}
impl<T> IO for T where T: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static {}
pub type Stream = Box<dyn IO>;

#[async_trait]
pub trait ProtocolConnector: Send + Sync {
    async fn connect(&self) -> io::Result<Stream>;
}

struct NopConnector {}

#[async_trait]
impl ProtocolConnector for NopConnector {
    async fn connect(&self) -> io::Result<Stream> {
        todo!()
    }
}
pub struct Error {}

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

pub type Result<T> = ::core::result::Result<T, Error>;
