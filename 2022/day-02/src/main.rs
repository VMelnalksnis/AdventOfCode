use core::str::FromStr;
use std::env;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let score = get_score_from_plays(&filepath);
    let score_result = get_score_from_result(&filepath);
    println!("Score: {}", score);
    println!("Score from result: {}", score_result);
}

fn get_score_from_plays(path: &str) -> u32 {
    get_score(path, &|x: &str| Round::from_str_plays(x))
}

fn get_score_from_result(path: &str) -> u32 {
    get_score(path, &|x: &str| Round::from_str_result(x))
}

fn get_score(path: &str, f: &dyn Fn(&str) -> Round) -> u32 {
    fs::read_to_string(path)
        .expect("File should exist")
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(f)
        .map(|x| x.get_score())
        .sum()
}

struct Round {
    opponent: Play,
    player: Play,
    result: RoundResult,
}

impl Round {
    fn new(opponent: Play, player: Play) -> Self {
        let result = match (opponent, player) {
            (Play::Rock, Play::Scissor)
            | (Play::Paper, Play::Rock)
            | (Play::Scissor, Play::Paper) => RoundResult::Lose,

            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissor)
            | (Play::Scissor, Play::Rock) => RoundResult::Win,

            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissor, Play::Scissor) => RoundResult::Draw,
        };

        Round {
            opponent,
            player,
            result,
        }
    }

    fn from_result(opponent: Play, result: RoundResult) -> Self {
        let player = match (opponent, result) {
            (Play::Rock, r) => match r {
                RoundResult::Win => Play::Paper,
                RoundResult::Draw => Play::Rock,
                RoundResult::Lose => Play::Scissor,
            },
            (Play::Paper, r) => match r {
                RoundResult::Win => Play::Scissor,
                RoundResult::Draw => Play::Paper,
                RoundResult::Lose => Play::Rock,
            },
            (Play::Scissor, r) => match r {
                RoundResult::Win => Play::Rock,
                RoundResult::Draw => Play::Scissor,
                RoundResult::Lose => Play::Paper,
            },
        };

        Round {
            opponent,
            player,
            result,
        }
    }

    fn get_score(&self) -> u32 {
        let shape_score: u32 = match self.player {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissor => 3,
        };

        let round_score: u32 = match self.result {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        };

        shape_score + round_score
    }
}

impl Round {
    fn from_str_result(input: &str) -> Self {
        let mut args = input.split(" ");
        let opponent = args.next().expect(" ").parse::<Play>().expect(" ");
        let result = args.next().expect(" ").parse::<RoundResult>().expect(" ");

        Self::from_result(opponent, result)
    }

    fn from_str_plays(input: &str) -> Self {
        let mut args = input.split(" ");
        let opponent = args.next().expect(" ").parse::<Play>().expect(" ");
        let player = args.next().expect(" ").parse::<Play>().expect(" ");

        Self::new(opponent, player)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Play {
    type Err = ();

    fn from_str(input: &str) -> Result<Play, Self::Err> {
        match input {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissor),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum RoundResult {
    Win,
    Draw,
    Lose,
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(input: &str) -> Result<RoundResult, Self::Err> {
        match input {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = get_score_from_plays(TEST_PATH);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_input() {
        let result = get_score_from_plays(INPUT_PATH);
        assert_eq!(result, 17189);
    }

    #[test]
    fn test_result_sample() {
        let result = get_score_from_result(TEST_PATH);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_result_input() {
        let result = get_score_from_result(INPUT_PATH);
        assert_eq!(result, 13490);
    }
}
