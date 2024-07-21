use std::io;

use async_trait::async_trait;
use clap::{arg, value_parser, ArgMatches, Command};
use rk::{basic_command, ProtocolConnector, Stream};
use tokio::net::TcpStream;

pub fn tcp_subcommand() -> Command {
    return basic_command("tcp")
        .about("benchmark tcp accept")
        
        .arg(
            arg!(
                -H --header <headers> "Add header to request"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                --latency "Print latancy statistics"
            )
            .value_parser(value_parser!(bool)),
        )
        .arg(
            arg!(
                --timeout <timeout> "Socket/request timeout"
            )
            .value_parser(value_parser!(String)),
        );
}

#[derive(Clone)]
pub struct TcpConfig {}

pub struct TcpConnector {
    config: TcpConfig,
}
impl TcpConnector {
    pub fn new(c: &TcpConfig) -> Self {
        Self { config: c.clone() }
    }
}

#[async_trait]
impl ProtocolConnector for TcpConnector {
    type Connection = Stream;
    async fn connect(&self) -> anyhow::Result<Self::Connection> {
        todo!()
    }
}

pub fn parse_tcp_config(m: &ArgMatches) -> rk::Result<TcpConfig> {
    todo!()
}

pub fn do_tcp(m: &ArgMatches) -> anyhow::Result<()> {
    let config = parse_tcp_config(m)?;
    Ok(())
}
