extern crate core;

use std::env;
use std::fs;
use std::str::FromStr;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let one_by_one = move_one_by_one(&filepath);
    println!("One by one: {:?}", one_by_one);

    let at_once = move_all_at_once(&filepath);
    println!("At once: {:?}", at_once);
}

fn move_one_by_one(path: &str) -> Vec<char> {
    execute_command(path, Command::execute_one_by_one)
}

fn move_all_at_once(path: &str) -> Vec<char> {
    execute_command(path, Command::execute_all_at_once)
}

fn execute_command<F>(path: &str, move_function: F) -> Vec<char>
where
    F: Fn(&Command, &mut Stacks) -> (),
{
    let content = fs::read_to_string(path).expect("File should exist");
    let input = parse_input(&content);
    let mut stacks = input.0.parse::<Stacks>().expect("");
    let commands = input
        .1
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Command>().expect(""));

    for command in commands {
        move_function(&command, &mut stacks);
    }

    stacks
        .stacks
        .into_iter()
        .map(|mut stack| stack.pop())
        .filter(|x| x.is_some())
        .map(|x| x.expect(""))
        .collect()
}

fn parse_input(content: &str) -> (&str, &str) {
    let mut iter = content.split("\n\n").filter(|line| !line.is_empty());
    if let Some(stacks) = iter.next() {
        if let Some(commands) = iter.next() {
            return (stacks, commands);
        }
    }

    panic!("Failed to parse {:?}", iter);
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl FromStr for Stacks {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().collect::<Vec<&str>>();
        let line_count = lines.len();
        let stack_count = lines.last().expect("").replace(" ", "").len();
        let raw_stacks = lines
            .into_iter()
            .take(line_count - 1)
            .map(|line| {
                line.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|x| x[1])
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];

        for chars in raw_stacks.into_iter().rev() {
            for (i, char) in chars.into_iter().enumerate() {
                if char != ' ' {
                    stacks[i].push(char);
                }
            }
        }

        Ok(Stacks { stacks })
    }
}

#[derive(Debug)]
struct Command {
    from: usize,
    to: usize,
    count: usize,
}

impl Command {
    fn execute_one_by_one(&self, stacks: &mut Stacks) {
        for _ in 0..self.count + 1 {
            if let Some(value) = stacks.stacks[self.from].pop() {
                stacks.stacks[self.to].push(value);
            }
        }
    }

    fn execute_all_at_once(&self, stacks: &mut Stacks) {
        let from = &mut stacks.stacks[self.from];
        let mut values = from
            .drain(from.len() - self.count - 1..from.len())
            .collect();

        stacks.stacks[self.to].append(&mut values);
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let iter = input
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<usize>().expect("") - 1)
            .collect::<Vec<usize>>();

        Ok(Command {
            from: iter[1],
            to: iter[2],
            count: iter[0],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = move_one_by_one(TEST_PATH);
        assert_eq!(result, vec!['C', 'M', 'Z']);
    }

    #[test]
    fn test_input() {
        let result = move_one_by_one(INPUT_PATH);
        assert_eq!(result, vec!['T', 'L', 'F', 'G', 'B', 'Z', 'H', 'C', 'N']);
    }

    #[test]
    fn test_result_sample() {
        let result = move_all_at_once(TEST_PATH);
        assert_eq!(result, vec!['M', 'C', 'D']);
    }

    #[test]
    fn test_result_input() {
        let result = move_all_at_once(INPUT_PATH);
        assert_eq!(result, vec!['Q', 'R', 'Q', 'F', 'H', 'F', 'W', 'C', 'L']);
    }
}
