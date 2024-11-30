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
        let field = transpose(field);

        for line in field {
            let mut tilted: Vec<char> = line.chars().collect();
            for (i, c) in line.chars().enumerate().skip(1) {
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

            for (i, c) in tilted.into_iter().enumerate() {
                if c != 'O' {
                    continue;
                }

                acc += line.len() - i;
            }
        }
    }

    acc.to_string()
}

fn part2(_input: &str) -> String {
    todo!()
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
