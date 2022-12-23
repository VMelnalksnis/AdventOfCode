use std::str::FromStr;
use std::{env, fs};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let count = get_count_segments(&filepath, 2);
    println!("Count: {:?}", count);

    let count = get_count_segments(&filepath, 10);
    println!("Count: {:?}", count);
}

fn get_count_segments(path: &str, count: usize) -> usize {
    let content = fs::read_to_string(&path).expect("File should exist");
    let commands = content
        .lines()
        .map(|line| line.parse::<Command>().expect("Valid command"))
        .collect::<Vec<Command>>();

    let mut segments = vec![(0, 0); count];
    let mut tail_positions: Vec<(i32, i32)> = vec![(0, 0)];

    for command in commands {
        for _ in 0..command.steps {
            let mut head = &mut segments[0];
            // println!("Head {:?}", head);

            match command.direction {
                Direction::Left => head.0 = head.0 - 1,
                Direction::Right => head.0 = head.0 + 1,
                Direction::Up => head.1 = head.1 + 1,
                Direction::Down => head.1 = head.1 - 1,
            }

            for index in 0..segments.len() - 1 {
                let head_position = segments[index];
                let mut tail_position = &mut segments[index + 1];

                // println!(
                //     "{} {:?}; {} {:?} {:?}",
                //     index,
                //     head_position,
                //     index + 1,
                //     tail_position,
                //     command.direction,
                // );

                let y_diff = head_position.1 - tail_position.1;
                let x_diff = head_position.0 - tail_position.0;

                match (x_diff, y_diff) {
                    (2, 1) | (1, 2) | (2, 2) => {
                        tail_position.0 = tail_position.0 + 1;
                        tail_position.1 = tail_position.1 + 1;
                    }
                    (2, -1) | (2, -2) | (1, -2) => {
                        tail_position.0 = tail_position.0 + 1;
                        tail_position.1 = tail_position.1 - 1;
                    }
                    (-2, -1) | (-2, -2) | (-1, -2) => {
                        tail_position.0 = tail_position.0 - 1;
                        tail_position.1 = tail_position.1 - 1;
                    }
                    (-2, 1) | (-1, 2) | (-2, 2) => {
                        tail_position.0 = tail_position.0 - 1;
                        tail_position.1 = tail_position.1 + 1;
                    }
                    (2, 0) => {
                        tail_position.0 = tail_position.0 + 1;
                    }
                    (-2, 0) => {
                        tail_position.0 = tail_position.0 - 1;
                    }
                    (0, -2) => {
                        tail_position.1 = tail_position.1 - 1;
                    }
                    (0, 2) => {
                        tail_position.1 = tail_position.1 + 1;
                    }
                    (_, _) => {}
                }

                // println!(
                //     "{} {:?}; {} {:?} {:?}",
                //     index,
                //     head_position,
                //     index + 1,
                //     tail_position,
                //     command.direction,
                // );
            }

            // print(&segments);
            tail_positions.push(*segments.last().expect(""));
        }
    }

    tail_positions.sort();
    tail_positions.dedup();

    tail_positions.len()
}

fn print(segments: &Vec<(i32, i32)>) {
    let mut x_min = segments.iter().map(|s| s.0).min().expect("");
    if x_min > 0 {
        x_min = 0;
    }

    let mut x_max = segments.iter().map(|s| s.0).max().expect("");
    if x_max < 0 {
        x_max = 0;
    }

    x_max = x_max + 1;

    let mut y_min = segments.iter().map(|s| s.1).min().expect("");
    if y_min > 0 {
        y_min = 0;
    }

    let mut y_max = segments.iter().map(|s| s.1).max().expect("");
    if y_max < 0 {
        y_max = 0;
    }

    y_max = y_max + 1;

    for y in (y_min..y_max).rev() {
        for x in x_min - 1..x_max {
            let mut printed = false;
            for (index, point) in segments.iter().enumerate() {
                if point.0 == x && point.1 == y {
                    if index == 0 {
                        print!("H");
                        printed = true;
                    } else {
                        print!("{}", index);
                        printed = true;
                    }

                    break;
                }
            }
            if !printed {
                print!(".")
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    steps: i32,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(" ");
        if let Some(dir) = iter.next() {
            if let Some(count) = iter.next() {
                let direction = dir.parse::<Direction>().expect("Expected valid direction");
                let steps = count.parse::<i32>().expect("Expected valid length");

                return Ok(Command { direction, steps });
            }
        }

        return Err(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_segments_sample() {
        let result = get_count_segments(TEST_PATH, 2);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_segments_input() {
        let result = get_count_segments(INPUT_PATH, 2);
        assert_eq!(result, 6745);
    }

    #[test]
    fn test_segments_2_sample() {
        let result = get_count_segments(TEST_PATH, 10);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_segments_2_input() {
        let result = get_count_segments(INPUT_PATH, 10);
        assert_eq!(result, 2793);
    }
}
