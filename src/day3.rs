/*
 * Advent of Code Day 3
 * Part 1: For each line, we find the largest digit in the first n-1 digits (n = line_length). We then save the index i of the max and find the
 *         largest digit in i..n.
 * Part 2: Similar attempt, but we define the battery size dynamically. Now we need to look in the first n-12 digits as we need at least 1 possible
 *         digit for the reamining digit of the battery. As the largest digit will always have the most impact, all other digits are less important.
 */

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    println!("Day 3");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in &lines {
        sum1 += get_max_joltage_part1(line);
        sum2 += get_max_joltage_part2(line);
    }
    return (sum1, sum2);
}

fn get_max_joltage_part1(line: &String) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let mut max_1 = (0, 0);
    for i in 0..chars.len() - 1 {
        let current_string: String = String::from(chars.get(i).unwrap().to_string());
        let current: usize = current_string.parse::<usize>().unwrap();
        if current > max_1.0 {
            max_1 = (current, i);
        }
    }
    let mut max_2 = 0;
    for j in (max_1.1 + 1)..chars.len() {
        let current_string: String = String::from(chars.get(j).unwrap().to_string());
        let current: usize = current_string.parse::<usize>().unwrap();
        if current > max_2 {
            max_2 = current;
        }
    }
    return max_1.0 * 10 + max_2;
}

fn get_max_joltage_part2(line: &String) -> usize {
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
            let current_string: String = String::from(chars.get(i as usize).unwrap().to_string());
            let current: usize = current_string.parse::<usize>().unwrap();
            if current > current_max {
                last_position = i;
                current_max = current;
            }
        }
        max.push_str(&current_max.to_string());
    }
    return max.parse::<usize>().unwrap();
}
