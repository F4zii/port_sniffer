use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use utils::input_handler::is_flag_valid;
use utils::output_handler::print_data;

mod constants;

mod utils;

const MAX: u16 = 65535;

struct EnvArguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl EnvArguments {
    fn new(args: &[String]) -> Result<EnvArguments, String> {
        if args.len() < 2 {
            return Err("not enough arguments".to_string());
        } else if args.len() > 4 {
            return Err("too many arguments".to_string());
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(EnvArguments { flag: String::from(""), ipaddr, threads: 4 });
        } else {
            let flag = args[1].clone();
            if !(is_flag_valid(&flag)) {
                return Err(format!("Unkown flag found: {}", flag));
            }
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \n\r       -h or -help to show this help message");
                return Err("help".to_string());
            } else if &flag == "-h" || &flag == "help" {
                return Err("too many arguments".to_string());
            } else if &flag == "-j" {
                if args.len() < 4 {
                    return Err("No provided ip address".to_string());
                }
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6".to_string())
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number".to_string())
                };
                return Ok(EnvArguments { threads, flag, ipaddr });
            } else {
                return Err("invalid syntax".to_string());
            }
        }
    }
}

fn scan_addr(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}

        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let program = args[0].clone();
    let args = EnvArguments::new(&args).unwrap_or_else(
        |err| {
            if !err.contains("help") {
                eprintln!("{} problem parsing arguments: {}", program, err);
            }
            process::exit(0);
        }
    );

    let num_threads = args.threads;
    let addr = args.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan_addr(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }


    out.sort();
    print_data(out);


}