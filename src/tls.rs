use clap::{arg, value_parser, Command};
use rk::basic_command;

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
                -c --connections <connections>  "Connections to keep open"
            )
            .required(true)
            .value_parser(value_parser!(usize)),
            
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
