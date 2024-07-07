use clap::{arg, value_parser, Command};
use rk::basic_command;

pub fn tls_subcommand() -> Command {
    return basic_command("tls")
        .about("benchmark tls handshake only")
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
        .override_usage("rk [OPTIONS] [url] [COMMAND]")
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
