use std::io::{Read, stdin};

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

fn part1(input: &str) -> String {
    let (mut list1, mut list2) = parse_input(input);

    list1.sort();
    list2.sort();

    list1
        .into_iter()
        .zip(list2)
        .map(|(id1, id2)| id2.abs_diff(id1))
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (list1, list2) = parse_input(input);

    list1
        .into_iter()
        .map(|id1| list2.iter().filter(|&&id2| id2 == id1).sum::<u32>())
        .sum::<u32>()
        .to_string()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (id1, id2) = line.split_once("   ").unwrap();
        list1.push(id1.parse().unwrap());
        list2.push(id2.parse().unwrap());
    }

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "2367773");
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "21271939");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "31");
    }
}
