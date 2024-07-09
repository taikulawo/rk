use std::io;

use async_trait::async_trait;
use clap::{arg, value_parser, ArgMatches, Command};
use rk::{basic_command, ProtocolConnector, Stream};

pub fn tcp_subcommand() -> Command {
    return basic_command("tcp")
        .about("benchmark tcp accept")
        .arg(
            arg!(
                -c --connections <connections>  "Connections to keep open"
            )
            .required(true)
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(
                -t --threads <thread>  "Number of threads to use"
            )
            .required(true)
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(
                -d --duration <duration> "Duration of test"
            )
            .value_parser(value_parser!(String)),
        )
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

pub struct TcpConnector<T> {
    config: TcpConfig,
    inner: T,
}
impl<T> TcpConnector<T> {
    pub fn new(inner: T, c: &TcpConfig) -> Self {
        Self {
            config: c.clone(),
            inner,
        }
    }
}

#[async_trait]
impl<T> ProtocolConnector for TcpConnector<T>
where
    T: ProtocolConnector,
{
    async fn connect(&self) -> io::Result<Stream> {
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
