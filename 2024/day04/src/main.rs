use std::io::{Read, stdin};

static XMAS: &str = "XMAS";
static SAMX: &str = "SAMX";

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
    let grid = parse_input(input);

    let width = grid.first().unwrap().len();
    let height = grid.len();

    let mut acc = 0;

    // Horizontal
    for y in 0..height {
        for x in 0..=(width - XMAS.len()) {
            let candidate: String = grid[y][x..(x + XMAS.len())].iter().collect();
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
            }
        }
    }

    // Vertical
    for x in 0..width {
        for y in 0..=(height - XMAS.len()) {
            let mut candidate = String::new();
            for i in 0..XMAS.len() {
                candidate.push(grid[y + i][x]);
            }
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
            }
        }
    }

    // Diagonal (top left to bottom right)
    'row: for y in 0..height {
        'col: for x in 0..width {
            let mut candidate = String::new();
            for i in 0..XMAS.len() {
                if x + i >= width {
                    break 'col;
                }

                if y + i >= height {
                    break 'row;
                }

                candidate.push(grid[y + i][x + i]);
            }
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
            }
        }
    }

    // Diagonal (top right to bottom left)
    'row: for y in 0..height {
        'col: for x in (0..width).rev() {
            let mut candidate = String::new();
            for i in 0..XMAS.len() {
                if i > x {
                    break 'col;
                }

                if y + i >= height {
                    break 'row;
                }

                candidate.push(grid[y + i][x - i]);
            }
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
            }
        }
    }

    acc
}

fn part2(input: &str) -> u32 {
    let grid = parse_input(input);

    let width = grid.first().unwrap().len();
    let height = grid.len();

    let mut acc = 0;
    'row: for y in 0..height {
        'col: for x in 0..width {
            if x + 2 >= width {
                break 'col;
            }

            if y + 2 >= height {
                break 'row;
            }

            let tl = grid[x][y];
            let tr = grid[x + 2][y];
            let br = grid[x + 2][y + 2];
            let bl = grid[x][y + 2];
            let m = grid[x + 1][y + 1];

            if m == 'A'
                && ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
                && ((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M'))
            {
                acc += 1;
            }
        }
    }

    acc
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2344);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1815);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 9);
    }
}
