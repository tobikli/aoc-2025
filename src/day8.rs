/*
 * Advent of Code Day 8
 * Part 1:
 * Part 2:
 */

use std::{
    collections::{HashMap, HashSet},
    f64,
};

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    println!("Day 8");
    let junctions: HashSet<(usize, (i64, i64, i64))> = lines
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let split: Vec<&str> = x.split(',').collect();
            return (
                i,
                (
                    split[0].parse::<i64>().unwrap(),
                    split[1].parse::<i64>().unwrap(),
                    split[2].parse::<i64>().unwrap(),
                ),
            );
        })
        .collect();
    let sum1 = create_first_n_circuits(&junctions, 1000);
    let sum2 = 0;
    return (sum1, sum2);
}

fn create_first_n_circuits(junctions: &HashSet<(usize, (i64, i64, i64))>, n: i64) -> usize {
    let mut connections: HashMap<(usize, usize), f64> = HashMap::new();
    for _ in 0..n {
        let mut min_distance: f64 = f64::INFINITY;
        let mut min_pair = (0, 0);
        for (i, (id1, position1)) in junctions.iter().enumerate() {
            for (j, (id2, position2)) in junctions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let distance = distance(*position1, *position2);
                if distance < min_distance
                    && !connections.contains_key(&(*id1, *id2))
                    && !connections.contains_key(&(*id2, *id1))
                {
                    min_distance = distance;
                    min_pair = (*id1, *id2);
                }
            }
        }
        connections.insert(min_pair, min_distance);
    }
    return get_first_n_circuits_from_connections(connections, 3);
}

fn get_first_n_circuits_from_connections(
    connections: HashMap<(usize, usize), f64>,
    n: usize,
) -> usize {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    println!("This solution has a really bad time complexity, as we need to merge all possible circuits -- Take a look at Union Find Algorithm for a better one.");
    'outer: for ((one, two), _) in connections {
        for circuit in circuits.iter_mut() {
            if circuit.contains(&one) || circuit.contains(&two) {
                circuit.insert(one);
                circuit.insert(two);
                continue 'outer;
            }
        }
        circuits.push(HashSet::from([one, two]));
    }
    let mut changes = true;
    while changes {
        changes = false;
        'outer: for i in 0..circuits.len() {
            for j in (i + 1)..circuits.len() {
                if !circuits[i].is_disjoint(&circuits[j]) {
                    let merged = circuits[j].drain().collect::<Vec<_>>();
                    circuits[i].extend(merged);
                    circuits.remove(j);
                    changes = true;
                    break 'outer;
                }
            }
        }
    }
    let mut lengths: Vec<usize> = circuits.iter().map(|x| x.len()).collect::<Vec<usize>>();
    lengths.sort_by(|a, b| b.cmp(a));
    return lengths[..n].iter().fold(1, |acc, x| acc * x);
}

fn distance(x: (i64, i64, i64), y: (i64, i64, i64)) -> f64 {
    return ((i64::pow(y.0 - x.0, 2) + i64::pow(y.1 - x.1, 2) + i64::pow(y.2 - x.2, 2)) as f64)
        .sqrt();
}
