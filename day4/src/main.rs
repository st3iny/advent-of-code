use std::{env, fs::read_to_string, str::FromStr};

#[derive(Clone, Copy)]
struct Range(pub usize, pub usize);

impl Range {
    fn contains(self, range: Range) -> bool {
        self.0 <= range.0 && self.1 >= range.1
    }

    fn overlaps(self, range: Range) -> bool {
        (self.0 >= range.0 && self.0 <= range.1)
            || (self.1 >= range.0 && self.1 <= range.1)
            || (range.0 >= self.0 && range.0 <= self.1)
            || (range.1 >= self.0 && range.1 <= self.1)
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or("Line contains no slash")?;
        let start = start.parse().map_err(|_| "Start is not a number")?;
        let end = end.parse().map_err(|_| "End is not a number")?;
        Ok(Range(start, end))
    }
}

fn main() {
    let input = input();
    let output = match env::args().nth(1).as_deref() {
        Some("part1") => part1(&input),
        Some("part2") => part2(&input),
        _ => panic!("Invalid part"),
    };
    println!("{:?}", output);
}

fn input() -> String {
    read_to_string("input").unwrap()
}

fn part1(input: &str) -> usize {
    input
        .split_terminator('\n')
        .filter(|line| {
            let (range1, range2) = line.split_once(',').expect("Invalid line format");
            let range1: Range = range1.parse().expect("Invalid range format");
            let range2: Range = range2.parse().expect("Invalid range format");
            range1.contains(range2) || range2.contains(range1)
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .split_terminator('\n')
        .filter(|line| {
            let (range1, range2) = line.split_once(',').expect("Invalid line format");
            let range1: Range = range1.parse().expect("Invalid range format");
            let range2: Range = range2.parse().expect("Invalid range format");
            range1.overlaps(range2)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 582);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 893);
    }
}
