use std::{env, fs::read_to_string};

fn main() {
    let input = input();
    let output = match env::args().nth(1).as_deref() {
        Some("part1") => part1(&input),
        _ => panic!("Invalid part"),
    };
    println!("{}", output);
}

fn input() -> String {
    read_to_string("input").expect("Failed to read input")
}

fn part1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.split_terminator('\n') {
        let round = line.split_once(' ').expect("Invalid line format");
        score += match round {
            // Draw
            ("A", "X") => 3,
            ("B", "Y") => 3,
            ("C", "Z") => 3,

            // Win
            ("A", "Y") => 6,
            ("B", "Z") => 6,
            ("C", "X") => 6,

            // Lose
            _ => 0,
        };
        score += match round.1 {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!("Invalid action"),
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 11906);
    }
}
