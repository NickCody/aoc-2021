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

struct Diag {
    z_count: i32,
    o_count: i32
}


fn process<I>(lines: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut report = Vec::<Diag>::with_capacity(12);
    for _n in std::ops::RangeInclusive::new(0, 11) {
        report.push(Diag{z_count: 0, o_count:0});
    }

    println!("report length is {}", report.len());

    for line in lines {
        let diag = line.as_ref();
        for (i, c) in diag.chars().enumerate() {
            if c == '0' {
                report[i].z_count += 1;
            } else if c == '1' {
                report[i].o_count += 1;

            }
        }
    }

    let g = gamma_rate(&report);
    let e = epsilon_rate(&report);

    println!("g: {}, e: {}, p: {}", g, e, g*e);
}

fn gamma(p: usize, r: &Vec<Diag>) -> u32 {
    if r[p].o_count >= r[p].z_count {
        1
    } else {
        0
    }
}

fn epsilon(p: usize, r: &Vec<Diag>) -> u32 {
    if r[p].z_count > r[p].o_count {
        1
    } else {
        0
    }
}

fn gamma_rate(r: &Vec<Diag>) -> u32 {
    let mut s: u32 = 0;
    for (i, _d) in r.iter().enumerate() {
        if (gamma(i, r)) == 1 {
            s += u32::pow(2, 11-(i as u32));
        }
    }
    s
}

fn epsilon_rate(r: &Vec<Diag>) -> u32 {
    let mut s: u32 = 0;
    for (i, _d) in r.iter().enumerate() {
        if (epsilon(i, r)) == 1 {
            s += u32::pow(2, 11-(i as u32));
        }
    }
    s
}