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

struct Board {
    data: Vec<Vec<u64>>
}

fn process<I>(lines: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut first = false;
    let mut numbers: Vec<u64>;
    let mut board_line = 0;
    let mut current_board: Board;

    for line in lines {
        if first {
            numbers = line.as_ref().to_string().split(',').map(|x| x.parse::<u64>().unwrap()).collect();
            first = false;
        } else {
            if (board_line == 0) {
                
            } else {

            }
        }
    }
}
