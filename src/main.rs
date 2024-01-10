use regex::Regex;
use rlimit::{getrlimit, setrlimit, Resource};
use std::os::unix::process::CommandExt;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if a command line argument is provided
    if args.len() != 4 {
        eprintln!("Usage: {} <stack_size> <haystack> <pattern>", args[0]);
        std::process::exit(1);
    }

    // Get the regex pattern from the command line argument2
    let stack_size = args[1].parse::<u64>().unwrap();
    let limit = getrlimit(Resource::STACK).unwrap();
    if limit.0 != stack_size {
        setrlimit(Resource::STACK, stack_size, limit.1).unwrap();
        let error = std::process::Command::new(&args[0]).args(&args[1..]).exec();
        println!("Error calling exec: {}", error);
    } else {
        let haystack = &args[2];
        let pattern = &args[3];
        eval_pattern(haystack, pattern); 
    }
}

fn eval_pattern(haystack: &str, pattern: &str) {
    let regex = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error compiling regex pattern: {}", e);
            std::process::exit(1);
        }
    };
    if regex.is_match(&haystack) {
        println!("The haystack matches the regex pattern.");
    } else {
        println!("The haystack does not match the regex pattern.");
    }
}