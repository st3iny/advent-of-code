use std::io::{stdin, Read};

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
    let mut acc = 0;

    let mut fields = Vec::new();
    let mut current_field = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            fields.push(current_field.clone());
            current_field.clear();
            continue;
        }

        current_field.push(line);
    }
    if !current_field.is_empty() {
        fields.push(current_field);
    }

    for field in &fields {
        if let Some(axis) = find_vertical(field) {
            // println!("Found vertical axis between {} and {}", axis, axis + 1);
            acc += axis * 100;
            continue;
        }

        if let Some(axis) = find_horizontal(field) {
            // println!("Found horizontal axis between {} and {}", axis, axis + 1);
            acc += axis;
        }
    }

    acc.to_string()
}

fn find_vertical(field: &[impl PartialEq]) -> Option<usize> {
    for i in 1..field.len() {
        let mut found = true;
        let mut j = 0;
        while i - j > 0 {
            match (field.get(i - j - 1), field.get(i + j)) {
                (Some(row1), Some(row2)) if row1 != row2 => {
                    found = false;
                    break;
                }
                (None, _) | (_, None) => break,
                _ => {}
            }

            j += 1;
        }

        if found {
            return Some(i);
        }
    }

    None
}

fn find_horizontal(field: &[&str]) -> Option<usize> {
    find_vertical(&transpose(field))
}

fn transpose(field: &[&str]) -> Vec<String> {
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

fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), include_str!("../part1.solution.txt").trim());
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), include_str!("../part2.solution.txt").trim());
    }
    */
}
