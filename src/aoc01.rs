use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: aoc01 <filename>");
        process::exit(-1);
    }

    let filename = &args[1];

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            std::process::exit(1);
        }
    };

    let mut prev: i32 = -1;
    let mut increased: i32 = 0;
    if let Ok(lines) = read_lines(file) {
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

fn read_lines(file: File) -> io::Result<io::Lines<io::BufReader<File>>> {
    Ok(io::BufReader::new(file).lines())
}
