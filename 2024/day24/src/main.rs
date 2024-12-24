use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, stdin},
    rc::Rc,
    str::FromStr,
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

fn part1(input: &str) -> u64 {
    let mut gates: HashMap<String, Rc<RefCell<Gate>>> = HashMap::new();
    let mut initial_values: HashMap<String, bool> = HashMap::new();

    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (gate, value) = line.split_once(": ").unwrap();
        let value = match value {
            "0" => false,
            "1" => true,
            _ => unreachable!(),
        };

        assert!(initial_values.insert(gate.to_string(), value).is_none());
    }

    for line in lines {
        let (source, drain) = line.split_once(" -> ").unwrap();
        let mut source_parts = source.split(' ');
        let gate1 = source_parts.next().unwrap();
        let operation: Operation = source_parts.next().unwrap().parse().unwrap();
        let gate2 = source_parts.next().unwrap();

        let gate1 = gates
            .entry(gate1.to_string())
            .or_insert_with(|| {
                Rc::new(RefCell::new(Gate {
                    name: gate1.to_string(),
                    value: initial_values.get(gate1).copied(),
                    operation: Operation::None,
                    inputs: None,
                }))
            })
            .clone();
        let gate2 = gates
            .entry(gate2.to_string())
            .or_insert_with(|| {
                Rc::new(RefCell::new(Gate {
                    name: gate2.to_string(),
                    value: initial_values.get(gate2).copied(),
                    operation: Operation::None,
                    inputs: None,
                }))
            })
            .clone();
        let mut drain = gates
            .entry(drain.to_string())
            .or_insert_with(|| {
                Rc::new(RefCell::new(Gate {
                    name: drain.to_string(),
                    value: initial_values.get(drain).copied(),
                    operation: Operation::None,
                    inputs: None,
                }))
            })
            .borrow_mut();
        drain.inputs = Some((gate1, gate2));
        drain.operation = operation;
    }

    let mut zs: Vec<&String> = gates.keys().filter(|name| name.starts_with("z")).collect();
    assert!(zs.len() <= 64);
    zs.sort();

    let mut z = 0;
    for gate_name in zs.into_iter().rev() {
        let mut gate = gates.get(gate_name).unwrap().borrow_mut();
        assert!(gate.inputs.is_some());
        let value = match gate.evaluate() {
            true => 1,
            false => 0,
        };
        z = z << 1 | value;
    }

    println!("\n{z:b}\n");

    let mut sorted_gates: Vec<Rc<RefCell<Gate>>> = gates.into_values().collect();
    sorted_gates.sort_by_key(|gate| gate.borrow().name.clone());
    for gate in sorted_gates {
        let gate = gate.borrow();
        if gate.name.chars().next().unwrap() > 'x' {
            continue;
        }

        let value = match gate.value.unwrap() {
            true => 1,
            false => 0,
        };
        println!("{}: {}\n", gate.name, value);
    }

    z
}

fn part2(_input: &str) -> u64 {
    todo!()
}

struct Gate {
    name: String,
    value: Option<bool>,
    operation: Operation,
    inputs: Option<(Rc<RefCell<Gate>>, Rc<RefCell<Gate>>)>,
}

impl Gate {
    fn evaluate(&mut self) -> bool {
        let Some(inputs) = &mut self.inputs else {
            return self.value.unwrap();
        };

        if self.value.is_some() {
            return self.value.unwrap();
        }

        let before = self.value;
        let mut input1 = inputs.0.borrow_mut();
        let mut input2 = inputs.1.borrow_mut();
        let val1 = input1.evaluate();
        let val2 = input2.evaluate();
        let result = self.operation.compute(val1, val2);
        self.value = Some(result);

        println!(
            "{} {:?} {} -> {} [{:?} -> {}]",
            input1.name, self.operation, input2.name, self.name, before, result,
        );

        result
    }
}

#[derive(Clone, Debug)]
enum Operation {
    And,
    Or,
    Xor,
    None,
}

impl Operation {
    fn compute(&self, a: bool, b: bool) -> bool {
        match self {
            Operation::And => a && b,
            Operation::Or => a || b,
            Operation::Xor => a ^ b,
            Operation::None => unreachable!(),
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(format!("Invalid operation: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");
    static EXAMPLE_INPUT_2: &str = include_str!("../example-input-2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 57270694330992);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 4);
    }

    #[test]
    fn test_part1_example_2() {
        assert_eq!(part1(EXAMPLE_INPUT_2), 2024);
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
