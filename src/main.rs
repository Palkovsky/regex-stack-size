use rlimit::{getrlimit, setrlimit, Resource};
use std::os::unix::process::CommandExt;

use evalregex::eval_pattern;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if a command line arguments are provided
    if args.len() != 4 {
        eprintln!("Usage: {} <stack_size> <haystack> <pattern>", args[0]);
        std::process::exit(1);
    }

    // Get the regex pattern from the command line argument
    let stack_size = args[1].parse::<u64>().unwrap();
    let limit = getrlimit(Resource::STACK).unwrap();


    if limit.0 != stack_size {
        // Adjust the stack size if to the provided value.
        setrlimit(Resource::STACK, stack_size, limit.1).unwrap();
        let error = std::process::Command::new(&args[0]).args(&args[1..]).exec();
        println!("Error calling exec: {}", error);
    } else {
        // Evaluate a haystack against the pattern
        let haystack = &args[2];
        let pattern = &args[3];
        eval_pattern(haystack, pattern); 
    }
}
