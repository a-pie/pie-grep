use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // We use 'if let' rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does. 
    // The run function doesn’t return a value that we want to unwrap in the same way that Config::new returns the Config instance. 
    // Because run returns () in the success case, we only care about detecting an error, 
    // so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

