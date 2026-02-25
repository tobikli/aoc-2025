use std::fs::File;
use std::io::{BufReader, Lines};

pub fn main(mut lines: Lines<BufReader<File>>) {
    println!("Advent of Code Day 1");
    let ring_size: i32 = 100;
    let mut _pointer: i32 = 50;
    let mut _count = 0;
    for line in lines.map_while(Result::ok) {
        let direction = line.chars().next().unwrap();
        let amount_string = line.split_at(direction.len_utf8()).1;
        let mut amount: i32 = amount_string.parse::<i32>().unwrap();
        if direction == 'L' {
            amount *= -1;
        }
        _pointer = (_pointer + amount) % ring_size;
        if _pointer == 0 {
            _count += 1;
        }
    }
    println!("Result: {}", _count);
}
