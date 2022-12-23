use std::str::FromStr;
use std::{env, fs};
const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let count = get_score(&filepath);
    println!("Score: {:?}", count);

    draw(&filepath);
}

fn get_score(path: &str) -> i32 {
    let content = fs::read_to_string(&path).expect("File should exist");
    let mut operations = content
        .lines()
        .map(|line| line.parse().expect("Valid operation"))
        .rev()
        .collect::<Vec<Op>>();

    let mut cycle = 0;
    let mut current_op: Op = operations.pop().expect("");
    let mut wait = match current_op {
        Op::Noop => 0,
        Op::Add(_) => 1,
    };
    let mut value = 1;
    let mut score = 0;

    while operations.len() != 0 {
        cycle = cycle + 1;
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                score = score + value * cycle;
            }
            _ => {}
        }

        if wait != 0 {
            wait = wait - 1;
            continue;
        }

        if let Op::Add(x) = current_op {
            value = value + x;
        }

        current_op = operations.pop().expect("");
        wait = match current_op {
            Op::Noop => 0,
            Op::Add(_) => 1,
        };
    }

    score
}

fn draw(path: &str) {
    let content = fs::read_to_string(&path).expect("File should exist");
    let mut operations = content
        .lines()
        .map(|line| line.parse().expect("Valid operation"))
        .rev()
        .collect::<Vec<Op>>();

    let mut cycle = 0;
    let mut current_op: Op = operations.pop().expect("");
    let mut wait = match current_op {
        Op::Noop => 0,
        Op::Add(_) => 1,
    };
    let mut value = 1;

    while operations.len() != 0 {
        cycle = cycle + 1;
        let pixel = cycle % 40 - 1;

        if (value - 1) == pixel || value == pixel || (value + 1) == pixel {
            print!("#")
        } else {
            print!(".")
        }

        match pixel + 1 {
            0 => println!(),
            _ => {}
        }

        if wait != 0 {
            wait = wait - 1;
            continue;
        }

        if let Op::Add(x) = current_op {
            value = value + x;
        }

        current_op = operations.pop().expect("");
        wait = match current_op {
            Op::Noop => 0,
            Op::Add(_) => 1,
        };
    }
}

enum Op {
    Noop,
    Add(i32),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(" ");
        if let Some(cmd) = iter.next() {
            match cmd {
                "noop" => Ok(Op::Noop),
                "addx" => match iter.next() {
                    None => Err(()),
                    Some(value) => match value.parse::<i32>() {
                        Ok(x) => Ok(Op::Add(x)),
                        Err(_) => Err(()),
                    },
                },
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_segments_sample() {
        let result = get_score(TEST_PATH);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_segments_input() {
        let result = get_score(INPUT_PATH);
        assert_eq!(result, 14040);
    }

    #[test]
    fn test_segments_2_sample() {
        draw(TEST_PATH);
    }

    #[test]
    fn test_segments_2_input() {
        draw(INPUT_PATH);
    }
}
