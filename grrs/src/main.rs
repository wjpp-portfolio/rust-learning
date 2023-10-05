
use clap::Parser;

/// Search for a pattern in a file and display matching lines (like grep)
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}




fn main() {
    let args = Cli::parse();
    

    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(content) => {println!("file content: {}", content)},
        Err(error) => {println!("error {}", error)},
    }

    //for line in content.lines() {
    //    if line.contains(&args.pattern) {
    //        println!("{}", line);
    //    }
    //}
}
