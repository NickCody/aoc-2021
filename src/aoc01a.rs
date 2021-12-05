use std::env;
use std::fs::File;
use std::io::{self, BufRead, Stdin};

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

fn get_lines_file(file: File) -> io::Result<io::Lines<io::BufReader<File>>> {
    Ok(io::BufReader::new(file).lines())
}

fn get_lines_stdin(i: Stdin) -> io::Result<io::Lines<io::BufReader<Stdin>>> {
    Ok(io::BufReader::new(i).lines())
}

fn process<I>(lines: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut prev: i32 = -1;
    let mut increased: i32 = 0;

    for line in lines {
        let l = line.as_ref().parse::<i32>().unwrap();
        if prev != -1 && l > prev {
            increased += 1;
        }
        prev = l;
    }

    println!("{}", increased);
}