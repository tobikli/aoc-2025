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
            for x in 0..coordinates.get(y).expect("Input is corrupted!").len() {
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
        .expect("Input is corrupted!")
        .get(x as usize)
        .expect("Input is corrupted!")
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
