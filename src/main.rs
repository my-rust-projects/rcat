use std::env;
use rcat::Config;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rcat::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
