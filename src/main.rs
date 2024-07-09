use h1::{do_h1, root_subcommand, H1Config};
use h2::h2_subcommand;
use h3::h3_subcommand;
use tcp::tcp_subcommand;
use tls::tls_subcommand;
use url::Url;
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
        .version(VERSION)
        .subcommand(h2_subcommand())
        .subcommand(tls_subcommand())
        .subcommand(tcp_subcommand())
        .subcommand(h3_subcommand())
        .get_matches();
    match matches.subcommand() {
        Some(("h2", matches)) => {}
        Some(("h3", matches)) => {}
        Some(("tcp", matches)) => {}
        Some(("tls", matches)) => {
            let target: &String = matches.get_one("url").unwrap();
            let target = Url::parse(target).expect("should correct url");
        }
        _ => {
            // h1
            let target: &Url = matches.get_one("url").unwrap();
            let c = H1Config {
                url: target.clone(),
            };
            do_h1(c);
        }
    }
    // Continued program logic goes here...
}
