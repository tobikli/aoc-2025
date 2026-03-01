/*
 * Advent of Code Day 4
 * Part 1: We build a 2D matrix from the input lines. For each coordinate, we simply check if the adjacent coordinates are in bound and how many of
 *         them are free. If its less then 4, we can return 1.
 * Part 2: We simply repeat the solution from part 1 and keep track if we remove new rolls. We replace removed rolls with '.'. Once no new rolls are
 *         removed, we consider all possible rolls as removed.
 */

pub fn main(lines: Vec<String>) {
    println!("Day 4");
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut last_sum2: isize = -1;
    let mut coordinates: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
    while last_sum2 < sum2 {
        last_sum2 = sum2;
        let mut new_coordinates: Vec<Vec<char>> = coordinates.clone();
        for y in 0..coordinates.len() {
            for x in 0..coordinates.get(y).unwrap().len() {
                let result = roll_is_accessible(&coordinates, x as isize, y as isize);
                if last_sum2 == 0 {
                    sum1 += result;
                }
                sum2 += result;
                if result == 1 {
                    new_coordinates[y][x] = '.';
                }
            }
        }
        coordinates = new_coordinates;
    }
    println!("Result Part 1: {}", sum1);
    println!("Result Part 2: {}", sum2);
}

fn roll_is_accessible(coordinates: &Vec<Vec<char>>, x: isize, y: isize) -> isize {
    if coordinates
        .get(y as usize)
        .unwrap()
        .get(x as usize)
        .unwrap()
        != &'@'
    {
        return 0;
    }
    let mut adjacent_rolls = 0;
    for j in y - 1..=y + 1 {
        if j < 0 {
            continue;
        }
        for i in x - 1..=x + 1 {
            if i < 0 || (i == x && j == y) {
                continue;
            }
            if let Some(line) = coordinates.get(j as usize) {
                if let Some(value) = line.get(i as usize) {
                    if value == &'@' {
                        adjacent_rolls += 1;
                    }
                }
            }
        }
    }
    return if adjacent_rolls < 4 { 1 } else { 0 };
}
