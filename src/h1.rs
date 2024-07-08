use clap::{arg, value_parser, Command};
use rk::basic_command;
use url::Url;
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

pub struct H1Config {
    pub url: Url,
}

pub fn do_h1(c: H1Config) {}
