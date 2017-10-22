extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

//    let config = match Config::new(&args) {
//        Ok(c) => c,
//        Err(e) => {
//            println!("Error parsing arguments: {}", e);
//            process::exit(1);
//        }
//    };
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("FATAL: {}", e);
        process::exit(1);
    }
}

