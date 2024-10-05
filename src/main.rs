use std::env;
use std::process;

use RustDemo::Config;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = RustDemo::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
