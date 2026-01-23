use std::env;
use std::fs;
use std::io::{self, Read};


#[derive(Debug, Default)]
struct Flags {
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
    max_line_length: bool,
}



struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
    max_line_length: usize,
}

fn count_text(input: &str) -> Counts {
    let bytes = input.as_bytes().len();
    let chars = input.chars().count();
    let lines = input.lines().count();
    let words = input.split_whitespace().count();

    let max_line_length = input
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    Counts {
        lines,
        words,
        bytes,
        chars,
        max_line_length,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut flags = Flags::default();
    let mut filename: Option<String> = None;

    for arg in &args[1..] {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--lines" => flags.lines = true,
                "--words" => flags.words = true,
                "--bytes" => flags.bytes = true,
                "--chars" => flags.chars = true,
                "--max-line-length" => flags.max_line_length = true,
                "--help" => {
                    print_help();
                    return;
                }
                "--version" => {
                    println!("wc (my_wc) 0.1.0");
                    return;
                }
                _ => {
                    eprintln!("Unknown option: {}", arg);
                    std::process::exit(1);
                }
            }
        }
        else if arg.starts_with("-") {
            for ch in arg.chars().skip(1) {
                match ch {
                    'l' => flags.lines = true,
                    'w' => flags.words = true,
                    'c' => flags.bytes = true,
                    'm' => flags.chars = true,
                    'L' => flags.max_line_length = true,
                    _ => {
                        eprintln!("Unknown flag: -{}", ch);
                        std::process::exit(1);
                    }
                }
            }
        }
         else {
            filename = Some(arg.clone());
        }
    }

    let contents = match &filename {
        Some(name) => {
            if name == "-" {
                let mut input = String::new();
                io::stdin()
                    .read_to_string(&mut input)
                    .expect("Failed to read stdin");
                input
            }
            else {
                fs::read_to_string(name)
                    .expect("Failed to read file")
            }
        }
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .expect("Failed to read stdin");
            input
        }
    };

    let counts = count_text(&contents);

    print_counts(&counts, &flags, filename.as_deref());
}

fn print_help() {
    println!("Usage: wc [OPTION]... [FILE]...");
    println!("Print newline, word, and byte counts for each FILE.");
    println!("  -c, --bytes            print the byte counts");
    println!("  -m, --chars            print the character counts");
    println!("  -l, --lines            print the newline counts");
    println!("  -L, --max-line-length  print the maximum display width");
    println!("  -w, --words            print the word counts");
}



fn print_counts(counts: &Counts, flags: &Flags, filename: Option<&str>) {
    let mut printed_any = false;

    if flags.lines {
        print!("{:>8}", counts.lines);
        printed_any = true;
    }
    if flags.words {
        print!("{:>8}", counts.words);
        printed_any = true;
    }
    if flags.chars {
        print!("{:>8}", counts.chars);
        printed_any = true;
    }
    if flags.bytes {
        print!("{:>8}", counts.bytes);
        printed_any = true;
    }
    if flags.max_line_length {
        print!("{:>8}", counts.max_line_length);
        printed_any = true;
    }

    if !printed_any {
        print!("{:>8}{:>8}{:>8}", counts.lines, counts.words, counts.bytes);
    }

    if let Some(name) = filename {
        print!(" {}", name);
    }

    println!();
}

