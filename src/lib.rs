use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::error::Error;

const MAX_PORT: u16 = 65535;

pub struct Config {
    ipaddr: IpAddr,
    num_threads: u16
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } 

        if args.len() > 4 {
            return Err("Too many arguments");
        }

        let flag = args[1].clone();
        if flag.contains("-h") || flag.contains("-help") {
            if args.len() > 2 {
                return Err("Too many arguments");
            } 
            
            println!("Usage: <Options>... <IPADDR>
                     \r\n-j            specify the number of threads
                     \r-h, -help     show help");
            return Err("help");
        } else if flag.contains("-j") {
            if args.len() < 4 {
                return Err("Not enough arguments");
            }
            
            let ipaddr = match IpAddr::from_str(&args[3]) {
                Ok(r) => r,
                Err(_) => return Err("Not a valid IPADDR")
            };
            let num_threads = match args[2].parse::<u16>() {
                Ok(r) => r,
                Err(_) => return Err("Cannot parse number of threads")
            };
            
            return Ok(Config{ipaddr, num_threads});
        } 

        if let Ok(ipaddr) = IpAddr::from_str(&args[1]) {
            return Ok(Config{ipaddr, num_threads: 4});
        }

        return Err("Syntax error");
    }
}

fn scan(port: u16, ipaddr: IpAddr) -> bool {
    if let Ok(_) = TcpStream::connect((ipaddr, port)) {
        return true;
    }

    return false;
}

// Run the program
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut open_ports: Vec<u16> = vec![];
    for i in 0..MAX_PORT {
        match scan(i, config.ipaddr) {
            true => {
                open_ports.push(i);
                print!(".");
            },
            false => {}
        }
    }

    open_ports.sort();
    for i in open_ports {
        println!("Found open port: {}", i);
    }

    Ok(())
}
