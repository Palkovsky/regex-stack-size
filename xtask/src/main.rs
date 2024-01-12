use duct::cmd;
use std::fs::File;
use std::io::Read;

fn main() {
    // Compile the shared library, preserving the .stack_sizes section
    cmd!("cargo", "+nightly", "rustc", "--lib", "--crate-type", "cdylib", "--release", "--", "-C", "link-arg=-Wl,-Tkeep-stack-sizes.x", "-C", "link-arg=-N")
        .env("RUSTFLAGS", "-Z emit-stack-sizes")
        .run()
        .unwrap();


    // Read the compiled shared object
    let file_path = "target/release/libevalregex.so";
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}. Maybe the compilation failed?", err);
            std::process::exit(1);
        }
    };

    let mut buffer = Vec::new();
    if let Err(err) = file.read_to_end(&mut buffer) {
        eprintln!("Error reading file: {}", err);
        std::process::exit(1);
    }

    // Analyze the stack usage
    let functions = stack_sizes::analyze_executable(&buffer).unwrap();
    let mut results = Vec::new();
    for (_, function) in functions.defined.iter() {
        let stack_usage = function.stack().unwrap_or_default();
        for name in function.names() {
            results.push((name, stack_usage));
        }
    }

    // Sort in the ascending order by usage
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (function, stack_usage) in results {
        println!("{},{} bytes", function, stack_usage);
    }
}
