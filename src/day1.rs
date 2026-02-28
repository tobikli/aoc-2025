pub fn main(lines: Vec<String>) {
    println!("Day 1");
    let ring_size: i32 = 100;
    let mut pointer: i32 = 50;
    let mut count_1: u32 = 0;
    let mut count_2: u32 = 0;
    for line in &lines {
        let direction = line.chars().next().expect("Input is corrupted!");
        let amount_string = line.split_at(direction.len_utf8()).1;
        let mut amount: i32 = amount_string.parse::<i32>().expect("Input is corrupted!");
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
    println!("Result Part 1: {}", count_1);
    println!("Result Part 2: {}", (count_1 + count_2));
}
