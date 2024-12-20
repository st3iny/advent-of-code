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

fn part1(input: &str) -> u16 {
    parse_input(input)
        .filter(|report| {
            let (last, report) = report.split_first().unwrap();
            let mut last = *last;
            let mut decreased = None;

            for &level in report {
                let decreased = *decreased.get_or_insert(level < last);
                if level == last
                    || (decreased && level > last)
                    || (!decreased && level < last)
                    || level.abs_diff(last) > 3
                {
                    return false;
                }

                last = level;
            }

            true
        })
        .count() as u16
}

fn part2(input: &str) -> u16 {
    parse_input(input)
        .filter(|report| {
            'outer: for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);

                let mut last = report.remove(0);
                let mut decreased = None;

                for level in report {
                    let decreased = *decreased.get_or_insert(level < last);
                    if level == last
                        || (decreased && level > last)
                        || (!decreased && level < last)
                        || level.abs_diff(last) > 3
                    {
                        continue 'outer;
                    }

                    last = level;
                }

                return true;
            }

            false
        })
        .count() as u16
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<u16>> {
    input.lines().map(|report| {
        report
            .split(' ')
            .map(|level| level.parse().unwrap())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 371);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 426);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 4);
    }
}
