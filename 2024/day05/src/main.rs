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

fn part1(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);

    let mut acc = 0;
    for queue in updates {
        if check_rules(&rules, &queue) {
            assert_eq!(queue.len() % 2, 1);
            acc += queue[queue.len() / 2] as u32;
            eprintln!(
                "[{}] {}",
                queue[queue.len() / 2],
                queue
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
    }

    acc
}

fn check_rules(rules: &[(u8, u8)], queue: &[u8]) -> bool {
    for (i, &page1) in queue.iter().enumerate() {
        for rule in rules {
            if rule.0 != page1 {
                continue;
            }

            let Some((j, _)) = queue
                .iter()
                .enumerate()
                .find(|(_, page2)| rule.1 == **page2)
            else {
                eprintln!("Rule {rule:?} is missing second page in {queue:?}");
                continue;
            };

            assert_ne!(i, j);
            if j < i {
                eprintln!("Rule {rule:?} not valid for {queue:?}");
                return false;
            }
        }
    }

    true
}

fn part2(_input: &str) -> u32 {
    todo!()
}

fn parse_input(input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let mut lines = input.lines();

    let mut order_rules = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (lower, upper) = line.split_once('|').unwrap();
        order_rules.push((lower.parse().unwrap(), upper.parse().unwrap()));
    }

    let mut updates = Vec::new();
    for line in lines {
        updates.push(line.split(',').map(|page| page.parse().unwrap()).collect());
    }

    (order_rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4637);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 143);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1815);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 9);
    }
    */
}
