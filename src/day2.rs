use std::fs::File;
use std::io::{BufReader, Lines};

pub fn main(lines: Vec<String>) {
    println!("Day 2");
    let single_line = lines.get(0).unwrap();
    let ranges = single_line.split(",");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for range in ranges {
        let mut split = range.split("-");
        let start_string = split.next().unwrap();
        let end_string = split.next().unwrap();
        let start: u64 = start_string.parse::<u64>().unwrap();
        let end: u64 = end_string.parse::<u64>().unwrap();
        for n in start..=end {
            if number_is_invalid_part1(n) {
                sum1 += n;
            }
            if number_is_invalid_part2(n) {
                sum2 += n;
            }
        }
    }
    println!("Result Part 1: {}", sum1);
    println!("Result Part 2: {}", sum2);
}

fn number_is_invalid_part1(input: u64) -> bool {
    let input_string = input.to_string();
    let length = input_string.chars().count();
    if length % 2 != 0 {
        return false;
    }
    let split = input_string.split_at(length / 2);
    return split.0 == split.1;
}

fn number_is_invalid_part2(input: u64) -> bool {
    let input_string = input.to_string();
    let length = input_string.chars().count();
    for i in 1..=length / 2 {
        if length % i != 0 {
            continue;
        }
        let mut temp: Vec<String> = Vec::new();
        let mut string_copy = input_string.clone();
        while !string_copy.is_empty() {
            let rest = string_copy.split_off(i);
            temp.push(string_copy);
            string_copy = rest;
        }

        let pattern = temp.get(0).unwrap();
        if temp.iter().map(|x| x == pattern).all(|x| x) {
            return true;
        }
    }
    return false;
}
