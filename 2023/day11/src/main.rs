use std::{
    io::{stdin, Read},
    ops::Deref,
};

fn main() {
    let input = || {
        let mut input = String::new();
        stdin().lock().read_to_string(&mut input).unwrap();
        input
    };
    let solution = match std::env::args().nth(1).unwrap().deref() {
        "part1" => part1(&input()),
        "part2" => part2(&input()),
        _ => panic!("Expected argument part1 or part2"),
    };
    println!("{solution}");
}

#[derive(Debug, PartialEq)]
struct Galaxy(u64, u64);

fn part1(input: &str) -> String {
    accumulate_shortest_paths(input, 2)
}

fn part2(input: &str) -> String {
    accumulate_shortest_paths(input, 1000000)
}

fn accumulate_shortest_paths(input: &str, expansion_factor: u64) -> String {
    // Expand rows and colums
    let line_length = input.lines().next().unwrap().len();
    let mut expanded_rows = Vec::new();
    let mut expanded_cols: Vec<usize> = (0..line_length).collect();
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        assert_eq!(line_length, line.len());

        if !line.contains('#') {
            expanded_rows.push(row);
        }

        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                expanded_cols.retain(|&col2| col2 != col);
            }
        }
    }

    // Parse galaxies
    let mut galaxies = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                let rows_to_add = expanded_rows.iter().filter(|&&row2| row2 < row).count() as u64;
                let cols_to_add = expanded_cols.iter().filter(|&&col2| col2 < col).count() as u64;
                let row = (row as u64) + rows_to_add * (expansion_factor - 1);
                let col = (col as u64) + cols_to_add * (expansion_factor - 1);
                galaxies.push(Galaxy(row, col));
            }
        }
    }

    // Calculate shortes paths
    let mut acc = 0;

    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            assert_ne!(galaxy, other_galaxy);
            let shortest_path =
                other_galaxy.0.abs_diff(galaxy.0) + other_galaxy.1.abs_diff(galaxy.1);
            acc += shortest_path;
        }
    }

    acc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), include_str!("../part1.solution.txt").trim());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), include_str!("../part2.solution.txt").trim());
    }
}
