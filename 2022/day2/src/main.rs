use std::{env, fs::read_to_string};

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
            let round = line.split_once(' ').expect("Invalid line format");
            score(round)
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .map(|line| {
            let round = line.split_once(' ').expect("Invalid line format");
            let my_choice = match round {
                // Lose
                ("A", "X") => "Z",
                ("B", "X") => "X",
                ("C", "X") => "Y",

                // Draw
                ("A", "Y") => "X",
                ("B", "Y") => "Y",
                ("C", "Y") => "Z",

                // Win
                ("A", "Z") => "Y",
                ("B", "Z") => "Z",
                ("C", "Z") => "X",

                _ => unreachable!("Invalid round format"),
            };
            score((round.0, my_choice))
        })
        .sum()
}

fn score(round: (&str, &str)) -> i32 {
    let mut score = match round {
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
    };
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 11906);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 11186);
    }

    #[test]
    fn test_score() {
        // Lose
        assert_eq!(score(("A", "Z")), 3);
        assert_eq!(score(("B", "X")), 1);
        assert_eq!(score(("C", "Y")), 2);

        // Draw
        assert_eq!(score(("A", "X")), 4);
        assert_eq!(score(("B", "Y")), 5);
        assert_eq!(score(("C", "Z")), 6);

        // Win
        assert_eq!(score(("A", "Y")), 8);
        assert_eq!(score(("B", "Z")), 9);
        assert_eq!(score(("C", "X")), 7);
    }
}
