use std::env::args;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = args().nth(1).expect("no pattern given");
    let path = args().nth(2).expect("no path give");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    // :? used for verbose debug trait
    println!("Pattern: {:?}, path: {:?}", args.pattern, args.path)
}
