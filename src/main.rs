use std::path::PathBuf;

use h1::root_subcommand;
use h2::h2_subcommand;
use h3::h3_subcommand;
use tcp::tcp_subcommand;
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
        .version(VERSION)
        .subcommand(h2_subcommand())
        .subcommand(tls_subcommand())
        .subcommand(tcp_subcommand())
        .subcommand(h3_subcommand())
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches
        .get_one::<u8>("debug")
        .expect("Counts are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    // Continued program logic goes here...
}
