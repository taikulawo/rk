use clap::{arg, value_parser, Command};

pub fn h2_subcommand() -> Command {
    return Command::new("h2")
        .about("benchmark h2")
        .arg(
            arg!(
                -c --connections "Connections to keep open"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(true)
            .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(
                -d --duration "Duration of test"
            )
            .value_parser(value_parser!(String)),
        )
        .override_usage("rk [OPTIONS] [url] [COMMAND]")
        .arg(
            arg!(
                -H --header "Add header to request"
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
                --timeout "Socket/request timeout"
            )
            .value_parser(value_parser!(String)),
        );
}
