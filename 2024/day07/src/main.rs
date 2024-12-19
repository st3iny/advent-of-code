use std::io::{stdin, Read};

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
    input
        .lines()
        .filter_map(|line| {
            let (sum, operands) = parse_line(line);
            match backtrack(0, &operands, sum, false) {
                true => Some(sum),
                false => None,
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let (sum, operands) = parse_line(line);
            match backtrack(0, &operands, sum, true) {
                true => Some(sum),
                false => None,
            }
        })
        .sum()
}

fn backtrack(current: u64, tail: &[u64], target: u64, combine_op: bool) -> bool {
    let Some((head, tail)) = tail.split_first() else {
        return current == target;
    };

    if backtrack(current + head, tail, target, combine_op) {
        return true;
    }

    if backtrack(current * head, tail, target, combine_op) {
        return true;
    }

    if combine_op
        && backtrack(
            format!("{current}{head}").parse().unwrap(),
            tail,
            target,
            combine_op,
        )
    {
        return true;
    }

    false
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (sum, operands) = line.split_once(": ").unwrap();
    let sum: u64 = sum.parse().unwrap();
    let operands: Vec<u64> = operands.split(' ').map(|op| op.parse().unwrap()).collect();
    (sum, operands)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 44841372855953);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 11387);
    }
}
