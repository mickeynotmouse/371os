use std::env;
use std::fs;

fn main() {
    //get filename from command line
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
    eprintln!("Usage: wc [FILE]");
    std::process::exit(1);
    }

    let filename = &args[1];

    //read file
    let contents = fs::read_to_string(filename)
        .expect("Failed to read file");

    //count lines
    let line_count = contents.lines().count();

    //count words
    let word_count = contents.split_whitespace().count();

    //count bytes
    let byte_count = contents.as_bytes().len();

    //printing
    println!(
        "{:>7} {:>7} {:>7} {}",
        line_count,
        word_count,
        byte_count,
        filename
    );
}
