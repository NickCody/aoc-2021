use std::fs::File;
use std::io::{self, BufRead, Stdin};

pub fn get_lines_file(file: File) -> io::Result<io::Lines<io::BufReader<File>>> {
Ok(io::BufReader::new(file).lines())
}

pub fn get_lines_stdin(i: Stdin) -> io::Result<io::Lines<io::BufReader<Stdin>>> {
Ok(io::BufReader::new(i).lines())
}
