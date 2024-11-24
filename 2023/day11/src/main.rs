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
        "part2" => todo!(),
        _ => panic!("Expected argument part1 or part2"),
    };
    print!("{solution}");
}

#[derive(Debug, PartialEq)]
struct Galaxy(usize, usize);

fn part1(input: &str) -> String {
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
                expanded_cols.retain(|col2| *col2 != col);
            }
        }
    }

    // Parse galaxies
    let mut galaxies = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                let row = row + expanded_rows.iter().filter(|&&row2| row2 < row).count();
                let col = col + expanded_cols.iter().filter(|&&col2| col2 < col).count();
                galaxies.push(Galaxy(row, col));
            }
        }
    }

    // Calculate shortes paths
    let mut acc = 0usize;

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

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "9521776");
    }
}
