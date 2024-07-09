use std::io;

use anyhow::bail;
use async_trait::async_trait;
use clap::{arg, value_parser, ArgMatches, Command};
use rk::{basic_command, ProtocolConnector, Stream};
use url::Url;

use crate::{
    tcp::{parse_tcp_config, TcpConfig, TcpConnector},
    tls::{parse_tls_config, TlsConfig, TlsConnector},
};
static NAME: &'static str = env!("CARGO_PKG_NAME");
pub fn root_subcommand() -> Command {
    return basic_command(NAME)
        .subcommand_negates_reqs(true)
        .subcommand_required(false)
        .about(
            "By default we benchmark url using HTTP/1.1(works like wrk).\nOr you can use Commands to benchmark other protocol.",
        )
        .arg(
            arg!(
                -c --connections <connections>  "Connections to keep open"
            ).required(true)
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(
                -t --threads <thread>  "Number of threads to use"
            ).required(true)
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
pub struct H1Config {
    pub url: Url,
    pub tls_config: TlsConfig,
    pub tcp_config: TcpConfig,
}
struct Http1Connector {
    config: H1Config,
}

impl Http1Connector {}

impl Http1Connector {
    pub fn new(c: &H1Config) -> Self {
        Self { config: c.clone() }
    }
}

#[async_trait]
impl ProtocolConnector for Http1Connector {
    async fn connect(&self) -> io::Result<Stream> {
        todo!()
    }
}

fn parse_h1_config(m: &ArgMatches) -> anyhow::Result<H1Config> {
    let tls_config = parse_tls_config(&m)?;
    let tcp_config = parse_tcp_config(&m)?;
    let target: &Url = m.get_one("url").unwrap();
    let c = H1Config {
        url: target.clone(),
        tcp_config,
        tls_config,
    };
    Ok(c)
}

pub fn do_h1(m: &ArgMatches) -> anyhow::Result<()> {
    let c = parse_h1_config(m)?;
    let connector: Box<dyn ProtocolConnector> = match c.url.scheme() {
        "http" => {
            let h1 = Http1Connector::new(&c);
            let tcp_connector = TcpConnector::new(h1, &c.tcp_config);
            Box::new(tcp_connector)
        }
        "https" => {
            let h1 = Http1Connector::new(&c);
            let tls = TlsConnector::new(h1, &c.tls_config);
            let tcp = TcpConnector::new(tls, &c.tcp_config);
            Box::new(tcp)
        }
        scheme => bail!("unknown scheme for http1 connector {scheme}"),
    };
    Ok(())
}
