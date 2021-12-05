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


fn process<I>(lines: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut forward: i32 = 0;
    let mut depth: i32 = 0;

    for line in lines {
        let cmd = line.as_ref().split(' ').collect::<Vec<&str>>();
        let val = cmd[1].parse::<i32>().unwrap();

        if cmd[0] == "forward" {
            forward += val;
        } else if cmd[0] == "down" {
            depth += val;
        } else if cmd[0] == "up" {
            depth -= val;
        }
    }

    println!("forward * depth: {}", forward * depth);
}