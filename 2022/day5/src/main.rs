use std::{collections::VecDeque, env, fs::read_to_string};

use itertools::Itertools;
use regex::Regex;

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

fn part1(input: &str) -> String {
    let mut lines = input.lines();

    // Parse initial stacks
    let mut stacks = Vec::new();
    for line in lines.by_ref() {
        if line.chars().nth(1).unwrap().is_numeric() {
            break;
        }

        for (i, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let c = chunk.nth(1).expect("Invalid crate stack format");
            if !('A'..='Z').contains(&c) {
                continue;
            }

            while stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            let stack = &mut stacks[i];
            stack.push_back(c);
        }
    }

    // Skip empty separator line
    lines.next();

    // Parse and execute moves
    let command_regex =
        Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Failed to compile regex");
    for line in lines {
        let captures = command_regex
            .captures(line)
            .expect("Invalid command format");
        let amount: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let src: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let dest: usize = captures.get(3).unwrap().as_str().parse().unwrap();

        for _ in 0..amount {
            let c = stacks[src - 1].pop_front().expect("Stack is already empty");
            stacks[dest - 1].push_front(c);
        }
    }

    // Concatenate first crate of each stack
    stacks
        .into_iter()
        .map(|mut stack| stack.pop_front().unwrap())
        .collect()
}

fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), "JRVNHHCSJ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), "TODO");
    }
}
