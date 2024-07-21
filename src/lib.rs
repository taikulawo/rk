use core::fmt;
use std::{collections::HashMap, fmt::Display, io};

use async_trait::async_trait;
use clap::{arg, value_parser, ArgMatches, Command};
use tokio::io::{AsyncRead, AsyncWrite};
use url::Url;
pub static NAME: &'static str = env!("CARGO_PKG_NAME");
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
    type Connection;
    async fn connect(&self) -> anyhow::Result<Self::Connection>;
}
pub fn parse_http_header(m: &ArgMatches) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let values: Vec<&String> = match m.get_many("headers") {
        Some(v) => v.collect(),
        None => Vec::new(),
    };
    for v in values {
        let s: Vec<&str> = v.splitn(1, ":").collect();
        assert_eq!(s.len(), 2);
        headers.insert(s[0].to_string(), s[1].to_string());
    }
    headers
}
struct NopConnector {}

#[async_trait]
impl ProtocolConnector for NopConnector {
    type Connection = ();
    async fn connect(&self) -> anyhow::Result<Self::Connection> {
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
