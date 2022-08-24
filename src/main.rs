use std::env;
use std::process;

use port_sniffer::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        if !err.contains("help") {
            eprintln!("Error parsing arguments: {}", err);
        }
        process::exit(0);
    });

    if let Err(e) = port_sniffer::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
