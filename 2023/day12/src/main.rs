use std::{
    fmt::{Display, Write},
    io::{stdin, Read},
};

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

#[derive(Clone, Debug, PartialEq)]
enum Spring {
    Unknown(usize),
    Damaged(usize),
    Operational(usize),
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (n, c) = match self {
            Spring::Unknown(n) => (*n, '?'),
            Spring::Damaged(n) => (*n, '#'),
            Spring::Operational(n) => (*n, '.'),
        };
        for _i in 0..n {
            f.write_char(c)?;
        }
        Ok(())
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
struct SpringRow(Vec<Spring>);

impl SpringRow {
    fn checksum(&self) -> Vec<usize> {
        self.0
            .iter()
            .filter_map(|spring| match spring {
                Spring::Damaged(n) => Some(*n),
                _ => None,
            })
            .collect()
    }
}

impl Display for SpringRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in &self.0 {
            write!(f, "{spring}")?;
        }
        Ok(())
    }
}

peg::parser! {
    grammar spring_parser() for str {
        rule unknown() -> Spring
            = s:$(['?']+) { Spring::Unknown(s.len()) }

        rule damaged() -> Spring
            = s:$(['#']+) { Spring::Damaged(s.len()) }

        rule operational() -> Spring
            = s:$(['.']+) { Spring::Operational(s.len()) }

        rule spring() -> Spring
            = unknown() / damaged() / operational()

        pub rule row() -> SpringRow
            = s:spring()+ { SpringRow(s) }
    }
}

peg::parser! {
    grammar checksum_parser() for str {
        rule block() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule checksum() -> Vec<usize>
            = s:block() ++ "," { s }
    }
}

fn resolve_backtrack(
    input: &[char],
    current: &mut Vec<char>,
    n: usize,
    found: &mut impl FnMut(String),
) {
    static CHARS: [char; 2] = ['#', '.'];

    if current.len() == n {
        found(current.iter().collect());
        return;
    }

    let (&first, input) = input.split_first().unwrap();
    if first != '?' {
        current.push(first);
        resolve_backtrack(input, current, n, found);
        current.pop();
        return;
    }

    for &c in &CHARS {
        current.push(c);
        resolve_backtrack(input, current, n, found);
        current.pop();
    }
}

fn part1(input: &str) -> String {
    let mut acc = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (springs_raw, checksum_raw) = line.split_once(' ').unwrap();
        let checksum = checksum_parser::checksum(checksum_raw).expect("To parse checksum");
        let damaged_sum: usize = checksum.iter().sum();

        assert!(
            springs_raw.contains('?'),
            "Spring row does not contain unresolved springs",
        );

        let input: Vec<char> = springs_raw.chars().collect();
        let mut current = Vec::new();
        let n = input.len();
        resolve_backtrack(&input, &mut current, n, &mut |permutation_raw| {
            // No need to parse if there aren't enough operational springs anyway
            let damaged = permutation_raw.chars().filter(|&c| c == '#').count();
            if damaged != damaged_sum {
                return;
            }

            let permutation = spring_parser::row(&permutation_raw).expect("To parse spring row");
            if permutation.checksum() == checksum {
                acc += 1;
            }
        });
    }

    acc.to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), include_str!("../part1.solution.txt").trim());
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), include_str!("../part2.solution.txt").trim());
    }
    */
}
