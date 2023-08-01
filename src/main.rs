use clap::Parser;
use graby::Config;
use std::process;

fn main() {
    // Parsing the terminal arguments to the Config structure.
    let config = Config::parse();
    // Check for parsing errors.
    if let Err(e) = graby::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
