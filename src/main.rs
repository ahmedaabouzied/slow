use std::env;
use std::io;
use std::process;
use std::thread;
use std::time;

fn print_help() {
    println!(
        "slow
     A linux utility to slowly print contents of a file to stdout.
     It's intended to help with inspecting logs. You can do
     cat ./service.log | slow 0.2
     which will print one line every 200 milliseconds to be easier 
     for humansa to read.

     Usage:
     slow <interval>

     Example:
     cat ./service.log | slow 0.1
    "
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("error: missing interval");
        process::exit(1);
    }
    if args[1] == "--help" {
        print_help();
        return;
    }
    let interval_secs: f64 = match args[1].parse::<f64>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("error parsing interval `{}`: {}", args[1], e);
            process::exit(1);
        }
    };
    let interval = time::Duration::from_secs_f64(interval_secs);
    let lines_iter = io::stdin().lines();
    for line in lines_iter {
        match line {
            Ok(v) => {
                println!("{}", v);
                thread::sleep(interval);
            }
            Err(e) => {
                eprintln!("error printing line from stdin: {}", e);
                process::exit(1);
            }
        }
    }
}
