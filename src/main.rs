#![allow(unused)]
mod advent1;
mod advent2;
mod advent3;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(input) = args.get(1) {
        if let Ok(mut lines) = read_lines(input) {
            let lines_vec: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
            advent2::main(lines_vec);
        } else {
            println!("Error when reading input!")
        }
    } else {
        println!("Usage: {} input.txt", args[0]);
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
