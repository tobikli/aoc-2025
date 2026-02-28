use std::collections::HashSet;

pub fn main(lines: Vec<String>) {
    println!("Day 5");
    let cut = lines.iter().position(|x| x == "").unwrap();
    let mut fresh_ingredients_list = lines.clone();
    let mut all_ingredients_list = fresh_ingredients_list.split_off(cut);
    all_ingredients_list = all_ingredients_list.split_off(1);
    let sum1 = parse_all_ingredients(&all_ingredients_list, &fresh_ingredients_list);
    let sum2 = get_amount_of_fresh_ingredients(&fresh_ingredients_list);
    println!("Result Part 1: {}", sum1);
    println!("Result Part 2: {}", sum2);
}

fn id_is_fresh(lines: &Vec<String>, id: usize) -> bool {
    for line in lines {
        let mut split = line.split("-");
        let start_string = split.next().unwrap();
        let end_string = split.next().unwrap();
        let start: usize = start_string.parse::<usize>().unwrap();
        let end: usize = end_string.parse::<usize>().unwrap();
        if id >= start && id <= end {
            return true;
        }
    }
    return false;
}

fn parse_all_ingredients(ingredients: &Vec<String>, fresh_ingredients: &Vec<String>) -> usize {
    let mut sum = 0;
    for line in ingredients {
        let id = line.parse::<usize>().unwrap();
        if id_is_fresh(fresh_ingredients, id) {
            sum += 1;
        }
    }
    return sum;
}

fn get_amount_of_fresh_ingredients(ingredients: &Vec<String>) -> usize {
    let mut ranges: HashSet<(usize, usize)> = ingredients
        .iter()
        .map(|x| {
            let mut split = x.split("-");
            let start_string = split.next().unwrap();
            let end_string = split.next().unwrap();
            let start: usize = start_string.parse::<usize>().unwrap();
            let end: usize = end_string.parse::<usize>().unwrap();
            return (start, end);
        })
        .collect();
    let mut changed_outer = true;
    let mut mutable_ranges = ranges.clone();
    while changed_outer {
        changed_outer = false;
        for &(start_1, end_1) in &ranges {
            for &(start_2, end_2) in &ranges {
                if start_1 < start_2 && end_1 > end_2 {
                    mutable_ranges.remove(&(start_1, end_1));
                    mutable_ranges.remove(&(start_2, end_2));
                    mutable_ranges.insert((start_1, end_1));
                    changed_outer = true;
                    break;
                } else if start_1 < start_2 && end_1 >= start_2 {
                    mutable_ranges.remove(&(start_1, end_1));
                    mutable_ranges.remove(&(start_2, end_2));
                    mutable_ranges.insert((start_1, end_2));
                    changed_outer = true;
                    break;
                } else if end_1 > end_2 && start_1 <= end_2 {
                    mutable_ranges.remove(&(start_1, end_1));
                    mutable_ranges.remove(&(start_2, end_2));
                    mutable_ranges.insert((start_2, end_1));
                    changed_outer = true;
                    break;
                }
            }
        }
        ranges = mutable_ranges.clone();
    }
    return ranges
        .iter()
        .map(|(s, e)| e - s + 1)
        .fold(0, |acc, x| acc + x);
}
