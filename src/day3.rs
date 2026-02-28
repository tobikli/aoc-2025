use std::fs::File;
use std::io::{BufReader, Lines};
use std::thread::current;

pub fn main(lines: Vec<String>) {
    println!("Day 3");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in &lines {
        sum1 += get_max_joltage_part1(line);
        sum2 += get_max_joltage_part2(line);
    }
    println!("Result Part 1: {}", sum1);
    println!("Result Part 2: {}", sum2);
}

fn get_max_joltage_part1(line: &String) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let mut max = 0;
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            let mut current_string: String =
                String::from(chars.get(i).unwrap().to_string());
            current_string.push(*chars.get(j).unwrap());
            let current: u32 = current_string.parse::<u32>().unwrap();
            if current > max {
                max = current;
            }
        }
    }
    return max;
}

fn get_max_joltage_part2(line: &String) -> u64 {
    let chars: Vec<char> = line.chars().collect();
    let line_length = chars.len();
    let mut max: String = "".to_string();
    let battery_size = 12;
    let mut last_position: isize = -1;
    for _ in 0..battery_size {
        let mut current_max = 0;
        for i in
            (last_position + 1)..(line_length - battery_size + 1 + max.chars().count()) as isize
        {
            let mut current_string: String = String::from(
                chars
                    .get(i as usize)
                    .unwrap()
                    .to_string(),
            );
            let current: u32 = current_string.parse::<u32>().unwrap();
            if current > current_max {
                last_position = i;
                current_max = current;
            }
        }
        max.push_str(&current_max.to_string());
    }
    return max.parse::<u64>().unwrap();
}
