extern crate clap;
extern crate simple_http;

use clap::{App, Arg, ArgMatches};
use simple_http::{run, Config};
use std::process;

fn main() {
    let matches = App::new("simple-http")
        .version("0.1.0")
        .author("Pierre-Marie Dartus <dartus.pierremarie@gmail.com>")
        .about("Simplistic experimental HTTP server")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Port to run the server on")
                .default_value("8080"),
        ).arg(
            Arg::with_name("path")
                .index(1)
                .help("Path to serve")
                .default_value("."),
        ).get_matches();

    let config = parse_cli_args(matches);

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    };
}

fn parse_cli_args(matches: ArgMatches) -> Config {
    let port_str = matches.value_of("port").unwrap();

    let port = port_str.parse::<u16>().unwrap_or_else(|_| {
        eprintln!("Invalid port parameter: {}", port_str);
        process::exit(1);
    });

    let path = matches.value_of("path").unwrap().to_owned();

    Config { port, path }
}
