use std::{
    collections::HashMap,
    io::{stdin, Read},
};

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
    calculate_load(&tilt_north(&parse_input(input))).to_string()
}

fn part2(input: &str) -> String {
    let mut field = parse_input(input);

    let target = 1000000000;
    let mut step = 0;
    let mut seen = HashMap::new();
    while step < target {
        field = tilt_north(&field);
        field = tilt_west(&field);
        field = tilt_south(&field);
        field = tilt_east(&field);

        step += 1;

        match seen.get(&field) {
            Some(seen_at) => {
                let cycle_length = step - seen_at;
                let cycles_to_skip = (target - step) / cycle_length;
                step += cycles_to_skip * cycle_length;
            }
            None => {
                seen.insert(field.clone(), step);
            }
        };
    }

    calculate_load(&field).to_string()
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            Some(line.to_string())
        })
        .collect()
}

fn tilt_north(field: &[String]) -> Vec<String> {
    transpose(&tilt_left(&transpose(field), false))
}

fn tilt_west(field: &[String]) -> Vec<String> {
    tilt_left(field, false)
}

fn tilt_south(field: &[String]) -> Vec<String> {
    transpose(&tilt_left(&transpose(field), true))
}

fn tilt_east(field: &[String]) -> Vec<String> {
    tilt_left(field, true)
}

fn tilt_left(field: &[String], invert: bool) -> Vec<String> {
    let mut tilted_field = Vec::with_capacity(field.len());

    for line in field {
        let mut line: Vec<char> = line.chars().collect();
        if invert {
            line.reverse();
        }

        let mut tilted: Vec<char> = line.clone();
        for (i, c) in line.into_iter().enumerate().skip(1) {
            if c != 'O' {
                continue;
            }

            let mut new_pos = None;
            for j in (0..i).rev() {
                if tilted[j] != '.' {
                    break;
                }

                new_pos = Some(j);
            }

            if let Some(new_pos) = new_pos {
                assert_eq!(tilted[new_pos], '.');
                tilted[new_pos] = 'O';
                tilted[i] = '.'
            }
        }

        if invert {
            tilted.reverse();
        }

        tilted_field.push(tilted.into_iter().collect());
    }

    tilted_field
}

fn transpose(field: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let row_len = field.first().expect("Field to have at least one row").len();
    for x in 0..row_len {
        let mut col = String::with_capacity(field.len());
        for row in field {
            assert_eq!(row.len(), row_len);
            col.push(row.chars().nth(x).unwrap())
        }
        out.push(col)
    }

    assert_eq!(out.len(), row_len);
    assert_eq!(out[0].len(), field.len());

    out
}

fn calculate_load(field: &[String]) -> usize {
    let mut acc = 0;
    for line in transpose(field) {
        for (i, c) in line.chars().enumerate() {
            if c != 'O' {
                continue;
            }

            acc += line.len() - i;
        }
    }

    acc
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
