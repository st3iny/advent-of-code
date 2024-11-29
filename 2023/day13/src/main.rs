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
    solution(input, find_vertical, find_horizontal)
}

fn part2(input: &str) -> String {
    solution(input, find_vertical_part2, find_horizontal_part2)
}

fn solution<V, H>(input: &str, find_vertical: V, find_horizontal: H) -> String
where
    V: Fn(&[String]) -> Option<usize>,
    H: Fn(&[String]) -> Option<usize>,
{
    let mut acc = 0;

    let mut fields = Vec::new();
    let mut current_field = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            fields.push(current_field.clone());
            current_field.clear();
            continue;
        }

        current_field.push(line.to_string());
    }
    if !current_field.is_empty() {
        fields.push(current_field);
    }

    for field in &fields {
        if let Some(axis) = find_vertical(field) {
            acc += axis * 100;
            continue;
        }

        if let Some(axis) = find_horizontal(field) {
            acc += axis;
        }
    }

    acc.to_string()
}

fn find_vertical(field: &[String]) -> Option<usize> {
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

fn find_horizontal(field: &[String]) -> Option<usize> {
    find_vertical(&transpose(field))
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

fn find_vertical_part2(field: &[String]) -> Option<usize> {
    // Don't accept axis from part 1 as there is at least one smudge to fix
    let original_axis = find_vertical(field);
    let new_axis = find_vertical_part2_rec(field, false, original_axis);
    if original_axis.is_some() {
        assert_ne!(original_axis, new_axis, "Should not find axis from part 1");
    }

    new_axis
}

fn find_vertical_part2_rec(
    field: &[String],
    smudge_fixed: bool,
    original_axis: Option<usize>,
) -> Option<usize> {
    for i in 1..field.len() {
        let mut found = true;
        let mut j = 0;
        while i - j > 0 {
            let row1_index = i - j - 1;
            match (field.get(row1_index), field.get(i + j)) {
                (Some(row1), Some(row2)) if row1 != row2 => {
                    if smudge_fixed || diff(row1, row2) > 1 {
                        found = false;
                        break;
                    }

                    let mut fixed_field = field.to_vec();
                    fixed_field[row1_index] = row2.clone();
                    if let Some(axis) = find_vertical_part2_rec(&fixed_field, true, original_axis) {
                        return Some(axis);
                    }
                }
                (None, _) | (_, None) => break,
                _ => {}
            }

            j += 1;
        }

        if found && original_axis != Some(i) {
            return Some(i);
        }
    }

    None
}

fn diff(row1: &str, row2: &str) -> usize {
    assert_eq!(row1.len(), row2.len());
    row1.chars()
        .zip(row2.chars())
        .filter(|(a, b)| a != b)
        .count()
}

fn find_horizontal_part2(field: &[String]) -> Option<usize> {
    find_vertical_part2(&transpose(field))
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
