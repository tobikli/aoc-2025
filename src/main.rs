use std::env;
use std::process::exit;
use aoc25::read_lines;
use aoc25::day1;
use aoc25::day2;
use aoc25::day3;
use aoc25::day4;
use aoc25::day5;
use aoc25::day6;
use aoc25::day7;

fn main() {
    println!("Advent of Code 2025 in Rust © Tobias Klingenberg");
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        help(&args[0]);
        exit(1);
    }
    let day: u32 = args.get(1).unwrap().parse().expect("Day must be a number!");
    let input = args.get(2).unwrap();
    if let Ok(lines) = read_lines(input) {
        let lines_vec: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
        let (part1, part2) = match day {
            1 => day1::solve(lines_vec),
            2 => day2::solve(lines_vec),
            3 => day3::solve(lines_vec),
            4 => day4::solve(lines_vec),
            5 => day5::solve(lines_vec),
            6 => day6::solve(lines_vec),
            7 => day7::solve(lines_vec),
            _ => panic!("Day {} is not (yet) implemented!", day),
        };
        println!("Result Part 1: {}", part1);
        println!("Result Part 2: {}", part2);
    } else {
        eprintln!("Error when reading input!")
    }
}

fn help(program: &String) {
    eprintln!("Usage: {} <day> <input.txt>", program);
}
