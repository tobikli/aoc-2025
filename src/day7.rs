/*
 * Advent of Code Day 7
 * Part 1: We create a 2D Matrix, we then iterate over each row. If we find the starting beam 'S', we set the beam for the next line to the index. For the
 *         remaining rows, we check if we hit a '^' and if we have a beam in the previous row. If yes, we add a split count and add 2 new beams, removing the
 *         old one.
 * Part 2: We use an recursive approach. For each beam, we recursively create a new beam. Each new beam is considered a new timeline. We find the starting position
 *         of the first beam and start from there. Furthermore, we cache already known timelines, as execution time would be exponential otherwise.
 */

use std::collections::{HashMap, HashSet};

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    println!("Day 7");
    let coordinates: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
    let sum1 = get_splits_part1(&coordinates);
    let sum2 = get_splits_part2(&coordinates);
    return (sum1, sum2);
}

fn get_splits_part1(coordinates: &Vec<Vec<char>>) -> usize {
    let mut splits = 0;
    let mut beams_this: HashSet<usize> = HashSet::new();
    let mut beams_next: HashSet<usize> = HashSet::new();
    for row in coordinates {
        for (j, column) in row.iter().enumerate() {
            match column {
                &'S' => {
                    beams_next.insert(j);
                    break;
                }
                &'^' => {
                    if beams_this.contains(&j) {
                        splits += 1;
                        beams_next.remove(&j);
                        beams_next.insert(&j - 1);
                        beams_next.insert(&j + 1);
                    }
                }
                _ => (),
            }
        }
        beams_this = beams_next.clone();
    }
    return splits;
}

fn get_splits_part2(coordinates: &Vec<Vec<char>>) -> usize {
    let start_column = coordinates
        .get(0)
        .unwrap()
        .iter()
        .position(|x| x == &'S')
        .unwrap();
    let rows = coordinates.len();
    let cols = coordinates[0].len();

    fn traverse(
        coordinates: &Vec<Vec<char>>,
        rows: usize,
        cols: usize,
        row: usize,
        column: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if row == rows {
            return 1;
        }
        if let Some(&cached) = memo.get(&(row, column)) {
            return cached;
        }
        let ch = coordinates[row][column];
        let result = if ch == '^' {
            let left = if column > 0 {
                traverse(coordinates, rows, cols, row + 1, column - 1, memo)
            } else {
                1
            };
            let right = if column + 1 < cols {
                traverse(coordinates, rows, cols, row + 1, column + 1, memo)
            } else {
                1
            };
            left + right
        } else {
            traverse(coordinates, rows, cols, row + 1, column, memo)
        };
        memo.insert((row, column), result);
        result
    }

    let mut memo = HashMap::new();
    traverse(coordinates, rows, cols, 1, start_column, &mut memo)
}
