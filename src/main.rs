#![allow(unused)]
mod day1;
mod day2;
mod day3;
mod day4;
use std::env;

fn main() {
    println!("Advent of Code 2025 in Rust © Tobias Klingenberg");
    println!("Program will panic on incorrect input!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        help(&args[0]);
        exit(1);
    }
    let day: u32 = args.get(1).unwrap().parse().expect("Day must be a number!");
    let input = args.get(2).unwrap();
    if let Ok(mut lines) = read_lines(input) {
        let lines_vec: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
        match day {
            1 => day1::main(lines_vec),
            2 => day2::main(lines_vec),
            3 => day3::main(lines_vec),
            4 => day4::main(lines_vec),
            _ => eprintln!("Day {} is not (yet) implemented!", day),
        }
    } else {
        eprintln!("Error when reading input!")
    }
}

fn help(program: &String) {
    eprintln!("Usage: {} <day> <input.txt>", program);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
