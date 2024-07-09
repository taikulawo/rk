use std::collections::HashMap;

use clap::{arg, value_parser, ArgMatches, Command};
use rk::{basic_command, parse_http_header};
use url::Url;

use crate::{
    tcp::{parse_tcp_config, TcpConfig},
    tls::{parse_tls_config, TlsConfig},
};

pub fn h2_subcommand() -> Command {
    return basic_command("h2")
        .about("benchmark h2")
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

pub struct H2Config {
    pub url: Url,
    pub headers: HashMap<String, String>,
    pub tls_config: TlsConfig,
    pub tcp_config: TcpConfig,
}

fn parse_h2_config(m: &ArgMatches) -> anyhow::Result<H2Config> {
    let tls_config = parse_tls_config(&m)?;
    let tcp_config = parse_tcp_config(&m)?;
    let target: &Url = m.get_one("url").unwrap();
    let headers = parse_http_header(m);
    let c = H2Config {
        url: target.clone(),
        tcp_config,
        tls_config,
        headers,
    };
    Ok(c)
}
