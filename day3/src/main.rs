use std::{collections::HashSet, env, fs::read_to_string};

use itertools::Itertools;

fn main() {
    let input = input();
    let output = match env::args().nth(1).as_deref() {
        Some("part1") => part1(&input),
        Some("part2") => part2(&input),
        _ => panic!("Invalid part"),
    };
    println!("{}", output);
}

fn input() -> String {
    read_to_string("input").expect("Failed to read input")
}

fn part1(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .map(|line| {
            let (compartment1, compartment2) = line.split_at(line.len() / 2);
            let compartment1 = HashSet::<char>::from_iter(compartment1.chars());
            let compartment2 = HashSet::<char>::from_iter(compartment2.chars());
            let shared_items = compartment1.intersection(&compartment2);
            shared_items.copied().map(priority).sum::<i32>()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .tuples()
        .map(|(rucksack1, rucksack2, rucksack3)| {
            let rucksack1 = HashSet::<char>::from_iter(rucksack1.chars());
            let rucksack2 = HashSet::<char>::from_iter(rucksack2.chars());
            let rucksack3 = HashSet::<char>::from_iter(rucksack3.chars());
            let shared_items = rucksack1
                .intersection(&rucksack2)
                .filter(|item| rucksack3.contains(item));
            shared_items.copied().map(priority).sum::<i32>()
        })
        .sum()
}

fn priority(item: char) -> i32 {
    if ('a'..='z').contains(&item) {
        (item as i32) - 96
    } else if ('A'..='Z').contains(&item) {
        (item as i32) - 65 + 27
    } else {
        panic!("Unexpected item: {}", item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 8298);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2708);
    }

    #[test]
    fn test_priority() {
        for (c, expected) in ('a'..='z').zip(1..26) {
            assert_eq!(priority(c), expected);
        }

        for (c, expected) in ('A'..='Z').zip(27..52) {
            assert_eq!(priority(c), expected);
        }
    }
}
