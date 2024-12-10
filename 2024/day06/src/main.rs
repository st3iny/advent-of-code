use std::{
    collections::HashSet,
    io::{Read, stdin},
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

fn part1(input: &str) -> usize {
    let (map, mut guard) = parse_input(input);

    let mut steps = HashSet::new();
    loop {
        let (dx, dy) = guard.direction.delta();

        guard.x += dx;
        guard.y += dy;

        if map.obstacle_at(guard.x, guard.y) {
            guard.x -= dx;
            guard.y -= dy;
            guard.direction.rotate_clockwise_90_deg();
            continue;
        }

        if guard.x < 0 || guard.x >= map.right || guard.y < 0 || guard.y >= map.bottom {
            break;
        }

        steps.insert((guard.x, guard.y));
    }

    steps.len()
}

fn part2(_input: &str) -> usize {
    todo!()
}

type Coordinate = i16;

#[derive(Debug)]
struct Map {
    obstacles: HashSet<(Coordinate, Coordinate)>,
    right: Coordinate,
    bottom: Coordinate,
}

impl Map {
    fn obstacle_at(&self, x: Coordinate, y: Coordinate) -> bool {
        self.obstacles.contains(&(x, y))
    }
}

#[derive(Debug)]
struct Guard {
    x: Coordinate,
    y: Coordinate,
    direction: Direction,
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Bottom,
    Right,
}

impl Direction {
    /// (dx, dy)
    fn delta(&self) -> (Coordinate, Coordinate) {
        match self {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Bottom => (0, 1),
            Direction::Right => (1, 0),
        }
    }

    fn rotate_clockwise_90_deg(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

fn parse_input(input: &str) -> (Map, Guard) {
    let mut obstacles = HashSet::new();
    let mut guard_pos = None;
    let mut guard_direction = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x: Coordinate = x.try_into().expect("Coordinate to not overflow");
            let y: Coordinate = y.try_into().expect("Coordinate to not overflow");
            match c {
                '#' => {
                    obstacles.insert((x, y));
                }
                '^' => {
                    guard_pos = Some((x, y));
                    guard_direction = Some(Direction::Up);
                }
                _ => {}
            }
        }
    }

    let map = Map {
        obstacles,
        right: input
            .lines()
            .next()
            .unwrap()
            .len()
            .try_into()
            .expect("Coordinate to not overflow"),
        bottom: input
            .lines()
            .count()
            .try_into()
            .expect("Coordinate to not overflow"),
    };
    let guard_pos = guard_pos.expect("Map to contain a guard");
    let guard = Guard {
        x: guard_pos.0,
        y: guard_pos.1,
        direction: guard_direction.expect("Map to contain a guard"),
    };
    (map, guard)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input.txt");
    static EXAMPLE_INPUT: &str = include_str!("../example-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5067);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 41);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1815);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 9);
    }
    */
}
