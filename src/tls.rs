use std::io;

use async_trait::async_trait;
use clap::{arg, value_parser, ArgMatches, Command};
use rk::{basic_command, ProtocolConnector, Stream};

pub fn tls_subcommand() -> Command {
    return basic_command("tls")
        .about("benchmark tls handshake only")
        .arg(
            arg!(
                --cipher <cipher> "like: ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                --"tls-version" "like: tlsv1.2;tlsv1.3"
            )
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                --"session-ticket" "Default: true. Enable session-ticket or not."
            ).default_value("true")
            .value_parser(value_parser!(bool)),
        )
        .arg(
            arg!(
                --handshake "Only benchmark tls handshake. connection will close after handshake complete"
            )
            .default_value("false")
            .value_parser(value_parser!(bool)),
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
pub struct TlsConfig {}

pub struct TlsConnector<T> {
    config: TlsConfig,
    inner: T,
}
impl<T> TlsConnector<T>
where
    T: ProtocolConnector,
{
    pub fn new(inner: T, c: &TlsConfig) -> Self {
        Self {
            config: c.clone(),
            inner,
        }
    }
}

#[async_trait]
impl<T> ProtocolConnector for TlsConnector<T>
where
    T: ProtocolConnector,
{
    type Connection = Stream;
    async fn connect(&self) -> anyhow::Result<Self::Connection> {
        todo!()
    }
}

pub struct TlsConnection {}

pub fn parse_tls_config(m: &ArgMatches) -> rk::Result<TlsConfig> {
    todo!()
}
