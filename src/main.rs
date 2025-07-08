use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Reading {0} for {1}", config.filepath, config.query);

    if let Err(err) = minigrep::run(config) {
        println!("Application Error: {err}");
        process::exit(1);
    }
}
