use std::fs::File;
use std::io::{BufReader, Lines};

pub fn main(mut lines: Lines<BufReader<File>>) {
    println!("Advent of Code Day 2");
    let single_line = lines.next().unwrap().unwrap();
    let ranges = single_line.split(",");
    let mut sum = 0;
    for range in ranges {
        let mut split = range.split("-");
        let start_string = split.next().unwrap();
        let end_string = split.next().unwrap();
        let start: u64 = start_string.parse::<u64>().unwrap();
        let end: u64 = end_string.parse::<u64>().unwrap();
        for n in start..=end {
            if number_is_invalid(n) {
                sum += n;
            }
        }
    }
    println!("Result: {}", sum);
}

fn number_is_invalid(input: u64) -> bool {
    let input_string = input.to_string();
    let length = input_string.chars().count();
    if length % 2 != 0 {
        return false;
    }
    let split = input_string.split_at(length / 2);
    return split.0 == split.1;
}
