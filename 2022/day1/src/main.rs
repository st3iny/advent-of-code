use std::{env, fs::read_to_string};

fn main() {
    match env::args().nth(1).as_deref() {
        Some("part1") => part1(),
        Some("part2") => part2(),
        _ => panic!("Invalid part"),
    }
}

fn part1() {
    let input = read_to_string("input").expect("Failed to read input");

    let mut calories = 0;
    let mut max_calories = 0;
    for line in input.split_terminator('\n') {
        if line.is_empty() {
            if calories > max_calories {
                max_calories = calories;
            }
            calories = 0;
            continue;
        }

        calories += line.parse::<i32>().expect("Failed to parse calories");
    }

    println!("{}", max_calories);
}

fn part2() {
    let input = read_to_string("input").expect("Failed to read input");

    let mut elves = Vec::new();
    let mut calories = 0;
    for line in input.split_terminator('\n') {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
            continue;
        }

        calories += line.parse::<i32>().expect("Failed to parse calories");
    }

    elves.sort();
    let top_three: i32 = elves.iter().rev().take(3).sum();
    println!("{}", top_three);
}
