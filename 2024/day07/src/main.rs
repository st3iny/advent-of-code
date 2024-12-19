use std::{
    collections::HashSet,
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

fn part1(input: &str) -> u64 {
    let mut valid = HashSet::new();
    for line in input.lines() {
        let (sum, operands) = line.split_once(": ").unwrap();
        let sum: u64 = sum.parse().unwrap();
        let operands: Vec<u64> = operands.split(' ').map(|op| op.parse().unwrap()).collect();
        backtrack(0, &operands, sum, &mut valid);
    }

    valid.into_iter().sum()
}

fn backtrack(current: u64, tail: &[u64], target: u64, results: &mut HashSet<u64>) {
    let Some((head, tail)) = tail.split_first() else {
        if current == target {
            results.insert(current);
        }
        return;
    };

    backtrack(current + head, tail, target, results);
    backtrack(current * head, tail, target, results);
}

fn part2(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2501605301465);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3749);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1793);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }
    */
}
