use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: aoc01 <filename>");
        process::exit(-1);
    }

    let filename = &args[1];

    let mut prev: i32 = -1;
    let mut increased: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.into_iter().flatten() {
            let l = line.parse::<i32>().unwrap();
            if prev != -1 && l > prev {
                increased += 1;
            }
            prev = l;
        }
    }

    println!("{}", increased);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}