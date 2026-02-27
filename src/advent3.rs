use std::fs::File;
use std::io::{BufReader, Lines};
use std::thread::current;

pub fn main(lines: Vec<String>) {
    println!("Advent of Code Day 3");
    let mut sum = 0;
    for line in &lines {
        sum += get_max_joltage(line);
    }
    println!("Result: {}", sum);
}

fn get_max_joltage(line: &String) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let mut max = 0;
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            let mut current_string: String = String::from(chars.get(i).unwrap().to_string());
            current_string.push(*chars.get(j).unwrap());
            let current: u32 = current_string.parse::<u32>().unwrap();
            if current > max {
                max = current;
            }
        }
    }
    return max;
}
