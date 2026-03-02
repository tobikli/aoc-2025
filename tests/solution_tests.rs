use aoc25::day1;
use aoc25::day2;
use aoc25::day3;
use aoc25::day4;
use aoc25::day5;
use aoc25::day6;
use aoc25::day7;
use aoc25::read_lines;

#[test]
fn test_solutions() {
    // =================================
    let functions = vec![
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
    ];
    let solutions: Vec<(usize, usize)> = vec![
        (1055, 6386),
        (28146997880, 40028128307),
        (17193, 171297349921310),
        (1578, 10132),
        (782, 353863745078671),
        (4719804927602, 9608327000261),
        (1628, 27055852018812),
    ];
    let days = 7;
    // =================================
    for i in 1..=days {
        let file_path = format!("input/input{}", i);
        let lines = read_lines(&file_path).expect(&format!("Failed to read file {}", file_path));
        let lines_vec: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
        let (part1, part2) = functions[i - 1](lines_vec);
        let (expected1, expected2) = solutions[i - 1];
        println!("Day {} - Part 1: {}", i, part1);
        println!("Day {} - Part 2: {}", i, part2);
        assert_eq!(part1, expected1);
        assert_eq!(part2, expected2);
    }
}
