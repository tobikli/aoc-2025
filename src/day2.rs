/*
 * Advent of Code Day 2
 * Part 1: We split the line at ',' and get the ranges by splitting again at '-'. A number is invalid if any sequence of digits is repeated
 *         exactly twice. There we can simply let it iterate over the range and check if the number as string can be splitted in half and if
 *         both halfs are identical.
 * Part 2: The digit sequence now can be repeated at LEAST two times. Therefore, we can iterate over the first half of the number and for each
 *         substring, we check if ALL substrings following the first are equal.
 */

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    println!("Day 2");
    let single_line = lines.get(0).unwrap();
    let ranges = single_line.split(",");
    let mut sum1: usize = 0;
    let mut sum2: usize = 0;
    for range in ranges {
        let mut split = range.split("-");
        let start_string = split.next().unwrap();
        let end_string = split.next().unwrap();
        let start: usize = start_string.parse::<usize>().unwrap();
        let end: usize = end_string.parse::<usize>().unwrap();
        for n in start..=end {
            if number_is_invalid_part1(n) {
                sum1 += n;
            }
            if number_is_invalid_part2(n) {
                sum2 += n;
            }
        }
    }
    return (sum1, sum2);
}

fn number_is_invalid_part1(input: usize) -> bool {
    let input_string = input.to_string();
    let length = input_string.chars().count();
    if length % 2 != 0 {
        return false;
    }
    let split = input_string.split_at(length / 2);
    return split.0 == split.1;
}

fn number_is_invalid_part2(input: usize) -> bool {
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
