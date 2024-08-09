// use std::env::args;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// pattern to look for
    pattern: String,
    /// the path to the file to read
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read the file");

    let result = std::fs::read_to_string("test.txt");
    match result {
        Ok(content) => {println!("File content: {}", content); }
        Err(error) => {panic!("Can't deal with {}, just exit here!", error ); }
        // Err(error) => {println!("Oh no: {}", error); }
    }

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }

}
// let pattern = args().nth(1).expect("no pattern given");
// let path = args().nth(2).expect("no path give");
//
// let args = Cli {
//     pattern: pattern,
//     path: std::path::PathBuf::from(path),
// };
    // :? used for verbose debug trait
    // println!("Pattern: {:?}, path: {:?}", args.pattern, args.path)
