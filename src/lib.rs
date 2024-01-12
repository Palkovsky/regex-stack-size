#[no_mangle]
pub fn eval_pattern(haystack: &str, pattern: &str) {
    let regex = match regex::Regex::new(&pattern) {
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
