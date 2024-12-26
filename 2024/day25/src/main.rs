use std::io::{Read, stdin};

const WIDTH: usize = 5;
const HEIGHT: u8 = 5;

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

fn part1(input: &str) -> usize {
    let schematics = parse_input(input);
    let locks = schematics
        .iter()
        .filter(|schematic| matches!(schematic, Schematic::Lock(..)));
    let keys: Vec<&Schematic> = schematics
        .iter()
        .filter(|schematic| matches!(schematic, Schematic::Key(..)))
        .collect();

    let mut acc = 0;
    for lock in locks {
        'keys: for key in &keys {
            for (l_col, k_col) in lock.pins().iter().zip(key.pins()) {
                if l_col + k_col > HEIGHT {
                    continue 'keys;
                }
            }

            acc += 1;
        }
    }

    acc
}

fn part2(_input: &str) -> usize {
    todo!()
}

fn parse_input(input: &str) -> Vec<Schematic> {
    let mut schematics = Vec::new();
    let mut current = None;
    for line in input.lines() {
        if line.is_empty() {
            schematics.push(current.take().expect("No blank lines at the beginning"));
            continue;
        }

        let first_row = current.is_none();
        let current = current.get_or_insert_with(|| match line {
            "#####" => Schematic::Lock([0, 0, 0, 0, 0]),
            "....." => Schematic::Key([HEIGHT, HEIGHT, HEIGHT, HEIGHT, HEIGHT]),
            _ => unreachable!(),
        });

        if first_row {
            continue;
        }

        for (i, c) in line.chars().enumerate() {
            match current {
                Schematic::Lock(pins) => {
                    if c == '#' {
                        pins[i] += 1;
                    }
                }
                Schematic::Key(pins) => {
                    if c == '.' {
                        pins[i] -= 1;
                    }
                }
            }
        }
    }

    if let Some(current) = current {
        schematics.push(current);
    }

    schematics
}

#[derive(Clone, Debug)]
enum Schematic {
    Lock([u8; WIDTH]),
    Key([u8; WIDTH]),
}

impl Schematic {
    fn pins(&self) -> &[u8; WIDTH] {
        match self {
            Self::Lock(pins) => pins,
            Self::Key(pins) => pins,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3451);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 44841372855953);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 11387);
    }
    */
}
