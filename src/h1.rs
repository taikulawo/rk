use clap::{arg, value_parser, Command};
static NAME: &'static str = env!("CARGO_PKG_NAME");
pub fn root_subcommand() -> Command {
    return Command::new(NAME)
        .about(
            "By default we benchmark using HTTP/1.1(works like wrk).\nOr you can use Commands to benchmark other protocols.",
        )
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
