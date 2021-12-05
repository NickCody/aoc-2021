extern crate aoc_common;

use std::env;
use std::fs::File;
use std::io::{self};
use aoc_common::{get_lines_file, get_lines_stdin};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        if let Ok(lines) = get_lines_stdin(io::stdin()) {
            process(lines.into_iter().flatten());
        }
    } else {
        let filename = &args[1];

        let file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {}", err);
                std::process::exit(1);
            }
        };
    
        if let Ok(lines) = get_lines_file(file) {
            process(lines.into_iter().flatten());
        }
    }
}

fn process<I>(_lines: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
}
