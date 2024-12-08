use std::{
    io::{Read, stdin},
    iter::repeat,
};

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
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid.first().unwrap().len();
    let height = grid.len();

    // To debug found candidates
    let mut positions = Vec::new();

    let mut acc = 0;

    // Horizontal
    for y in 0..height {
        for x in 0..=(width - XMAS.len()) {
            let candidate: String = grid[y][x..(x + XMAS.len())].iter().collect();
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
                positions.extend((x..(x + XMAS.len())).map(|x| (x, y, grid[y][x])));
            }
        }
    }

    // Vertical
    for x in 0..width {
        for y in 0..=(height - XMAS.len()) {
            let mut position_stack = Vec::new();
            let mut candidate = String::new();
            for i in 0..XMAS.len() {
                let c = grid[y + i][x];
                candidate.push(c);
                position_stack.push((x, y + i, c));
            }
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
                positions.extend_from_slice(&position_stack);
            }
        }
    }

    // Diagonal (top left to bottom right)
    'row: for y in 0..height {
        'col: for x in 0..width {
            let mut position_stack = Vec::new();
            let mut candidate = String::new();
            for i in 0..XMAS.len() {
                if x + i >= width {
                    break 'col;
                }

                if y + i >= height {
                    break 'row;
                }

                let c = grid[y + i][x + i];
                candidate.push(c);
                position_stack.push((x + i, y + i, c));
            }
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
                positions.extend_from_slice(&position_stack);
            }
        }
    }

    // Diagonal (top right to bottom left)
    'row: for y in 0..height {
        'col: for x in (0..width).rev() {
            let mut position_stack = Vec::new();
            let mut candidate = String::new();
            for i in 0..XMAS.len() {
                if i > x {
                    break 'col;
                }

                if y + i >= height {
                    break 'row;
                }

                let c = grid[y + i][x - i];
                candidate.push(c);
                position_stack.push((x - i, y + i, c));
            }
            if candidate == XMAS || candidate == SAMX {
                acc += 1;
                positions.extend_from_slice(&position_stack);
            }
        }
    }

    // Print grid with found candidates
    let mut grid_out = Vec::new();
    for _ in 0..height {
        grid_out.push(repeat('.').take(width).collect::<Vec<_>>());
    }
    for (x, y, c) in positions {
        grid_out[y][x] = c;
    }

    for y in 0..height {
        for x in 0..width {
            eprint!("{}", grid_out[y][x]);
        }
        eprintln!()
    }

    acc
}

fn part2(input: &str) -> u32 {
    todo!()
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

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 90669332);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 48);
    }
    */
}
