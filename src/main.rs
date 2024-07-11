use h1::{do_h1, root_subcommand};
use h2::h2_subcommand;
use h3::h3_subcommand;
use tcp::{do_tcp, tcp_subcommand};
use tls::tls_subcommand;
static VERSION: &str = env!("CARGO_PKG_VERSION");
mod h1;
mod h2;
mod h3;
mod tcp;
mod tls;
fn main() {
    // By default, we use h1 as top command, let rk act like wrk
    let matches = root_subcommand()
        .subcommand_negates_reqs(true)
        .disable_help_subcommand(true)
        .version(VERSION)
        .subcommand(h2_subcommand())
        .subcommand(tls_subcommand())
        .subcommand(tcp_subcommand())
        .subcommand(h3_subcommand())
        .get_matches();
    if let Err(err) = match matches.subcommand() {
        Some(("h2", matches)) => {
            todo!()
        }
        Some(("h3", matches)) => {
            todo!()
        }
        Some(("tcp", matches)) => do_tcp(matches),
        Some(("tls", matches)) => todo!(),
        _ => {
            // h1
            do_h1(&matches)
        }
    } {}
    // Continued program logic goes here...
}
