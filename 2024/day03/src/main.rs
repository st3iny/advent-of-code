use std::io::{Read, stdin};

use regex::Regex;

fn main() {
    let input = || {
        let mut input = String::new();
        stdin().lock().read_to_string(&mut input).unwrap();
        input
    };
    let solution = match std::env::args().nth(1).as_deref() {
        Some("part1") => part1(&input()),
        Some("part2") => part2(&input()),
        _ => panic!("Expected argument part1 or part2"),
    };
    println!("{solution}");
}

fn part1(input: &str) -> u32 {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Valid regex");

    input
        .lines()
        .flat_map(|program| {
            regex.captures_iter(program).map(|capture| {
                let x: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
                let y: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
                x * y
            })
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let regex = Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").expect("Valid regex");

    let mut acc = 0;
    let mut enabled = true;
    for program in input.lines() {
        for capture in regex.captures_iter(program) {
            match capture.get(0).unwrap().as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if !enabled {
                        continue;
                    }

                    let x: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
                    let y: u32 = capture.get(3).unwrap().as_str().parse().unwrap();
                    acc += x * y
                }
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 173419328);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 90669332);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 48);
    }
}
