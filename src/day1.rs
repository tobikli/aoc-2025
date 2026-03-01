/*
 * Advent of Code Day 1
 * Part 1: We split each line into 'Direction' and 'Amount' at the first character. The pointer points to the starting number 50.
 *         We then substract or add the amount in each line and use the modulo operator to get back into the ring set in ring_size.
 *         We count how many times the pointer points to 0.
 * Part 2: We need to count the times, the pointer passed a 0 as well. Therefore we additionaly check the quotient between the amount
 *         and the ring_size, to see how often we would have passed. Be careful of edge cases, as positive -> negative counts as passing 0
 *         but the quotient remains 0.
 */

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    println!("Day 1");
    let ring_size: isize = 100;
    let mut pointer: isize = 50;
    let mut count_1: usize = 0;
    let mut count_2: usize = 0;
    for line in &lines {
        let direction = line.chars().next().unwrap();
        let amount_string = line.split_at(direction.len_utf8()).1;
        let mut amount: isize = amount_string.parse::<isize>().unwrap();
        if direction == 'L' {
            amount *= -1;
        }
        let temp = pointer + amount;
        if temp < 0 || temp > ring_size {
            count_2 += (temp / ring_size).unsigned_abs();
            if temp % ring_size == 0 {
                count_2 -= 1;
            }
            if temp < 0 && pointer > 0 {
                count_2 += 1;
            }
        }
        pointer = if (temp % ring_size) < 0 {
            ring_size + (temp % ring_size)
        } else {
            temp % ring_size
        };
        if pointer == 0 {
            count_1 += 1;
        }
    }
    return (count_1, count_1 + count_2);
}
