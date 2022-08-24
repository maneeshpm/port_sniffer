use std::net::IpAddr;
use std::str::FromStr;

pub struct Config {
    flag: String,
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
            
            return Ok(Config{flag, ipaddr, num_threads});
        }

        return Err("Syntax error");
    }
}
