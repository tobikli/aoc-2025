/*
 * Advent of Code Day 6
 * Part 1: We create a 2D Matrix, and simply iterate over the rows instead of the columns. We store each operand in a Vector and the operator. Lastly
 *         we can fold the operand Vector to get the solution.
 * Part 2: We first try to get the indices of where each column is splitted. We can find it, if we check if at the given column index, every row has a
 *         whitespace. Given the indices of the column seperators, we can split each row into its numbers. As we read from right to left, we can reverse
 *         the string. Then we iterate once again over each row in the column. We build the operand by starting with a empty string and concatenating the
 *         first character of each "number" in the row to the operand and remove this first character from the matrix. Because we kept the whitespace characters
 *         each operand is correctly read from top to bottom and right to left. Lastly, we remove the whitespaces from the numbers and parse them.
 */

use regex::Regex;

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    println!("Day 6");
    let sum1 = get_sum_part1(&lines);
    let sum2 = get_sum_part2(&lines);
    return (sum1, sum2);
}

fn get_sum_part1(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    let regex = Regex::new(r"\s+").unwrap();
    let matrix: Vec<Vec<&str>> = lines
        .iter()
        .map(|x| regex.split(x).filter(|x| !x.is_empty()).collect())
        .collect();
    let columns = matrix.get(0).unwrap().len();
    let rows = matrix.len();
    for column in 0..columns {
        let mut operator = "";
        let mut operands: Vec<usize> = Vec::new();
        for row in 0..rows {
            let value = matrix.get(row).unwrap().get(column).unwrap();
            if row == rows - 1 {
                operator = value;
            } else {
                operands.push(value.parse::<usize>().unwrap());
            }
        }
        let row_sum = match operator {
            "+" => operands.iter().fold(0, |acc, x| acc + x),
            "*" => operands.iter().fold(1, |acc, x| acc * x),
            _ => 0,
        };
        sum += row_sum;
    }
    return sum;
}

fn get_sum_part2(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut split_indices: Vec<usize> = Vec::new();
    let columns = lines.get(0).unwrap().chars().count();
    let rows = lines.len();
    for column in 0..columns {
        let mut space_rows = 0;
        for row in 0..rows {
            if lines.get(row).unwrap().chars().nth(column).unwrap_or('_') == ' ' {
                space_rows += 1;
            }
        }
        if space_rows == rows {
            split_indices.push(column);
        }
    }
    let mut matrix: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            split_at_indices(line, &split_indices)
                .iter()
                .map(|s| s.chars().rev().collect())
                .collect()
        })
        .collect();
    let columns = matrix.get(0).unwrap().len();
    let rows = matrix.len();
    for column in 0..columns {
        let mut operator = String::new();
        let longest_number_len = matrix.get(0).unwrap().get(column).unwrap().chars().count();
        let mut operands: Vec<String> = vec![String::new(); longest_number_len];
        for i in 0..longest_number_len {
            for row in 0..rows {
                let mut value = matrix[row][column].clone();
                if row == rows - 1 {
                    operator = value.chars().filter(|x| x != &' ').collect();
                } else {
                    let saved_number = operands.get(i).unwrap();
                    let new_number = value.split_off(1);
                    matrix[row][column] = new_number;
                    operands[i] = saved_number.to_owned() + &value;
                }
            }
        }
        let operands_numbers: Vec<usize> = operands
            .iter()
            .map(|x| {
                x.chars()
                    .filter(|y| y != &' ')
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();
        let row_sum = match operator.as_str() {
            "+" => operands_numbers.iter().fold(0, |acc, x| acc + x),
            "*" => operands_numbers.iter().fold(1, |acc, x| acc * x),
            _ => 0,
        };
        sum += row_sum;
    }
    return sum;
}

fn split_at_indices<'a>(s: &'a str, indices: &[usize]) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for &i in indices {
        if i <= s.len() {
            result.push(&s[last..i]);
            last = i + 1;
        }
    }
    if last <= s.len() {
        result.push(&s[last..]);
    }
    result
}
