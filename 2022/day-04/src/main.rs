use core::ops::Range;
use std::env;
use std::fs;
use std::str::FromStr;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let count = get_contained_pair_count(&filepath);
    let overlap_count = get_overlapping_pair_count(&filepath);
    println!("Score: {}", count);
    println!("Score: {}", overlap_count);
}

fn get_contained_pair_count(path: &str) -> usize {
    get_count(path, Assignment::contains)
}

fn get_overlapping_pair_count(path: &str) -> usize {
    get_count(path, Assignment::overlaps)
}

fn get_count<F>(path: &str, assignment_filter: F) -> usize
where
    F: Fn(&Assignment) -> bool,
{
    fs::read_to_string(path)
        .expect("File should exist")
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Assignment>().expect("valid assignment"))
        .filter(assignment_filter)
        .count()
}

struct Assignment {
    first: Range<u32>,
    second: Range<u32>,
}

impl Assignment {
    fn contains(&self) -> bool {
        (self.first.start <= self.second.start && self.first.end >= self.second.end)
            || (self.first.start >= self.second.start && self.first.end <= self.second.end)
    }

    fn overlaps(&self) -> bool {
        (self.first.end > self.second.start && self.first.start <= self.second.start)
            || (self.second.end > self.first.start && self.second.start <= self.first.start)
    }
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.split(",").map(parse_str_to_range);
        if let Some(first) = iter.next() {
            if let Some(second) = iter.next() {
                return Ok(Assignment { first, second });
            }
        }

        Err(())
    }
}

fn parse_str_to_range(input: &str) -> Range<u32> {
    let mut iter = input.split("-").map(|c| c.parse::<u32>().expect("a digit"));
    if let Some(start) = iter.next() {
        if let Some(end) = iter.next() {
            return start..end + 1;
        }
    }

    panic!("Failed to parse {:?}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = get_contained_pair_count(TEST_PATH);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_input() {
        let result = get_contained_pair_count(INPUT_PATH);
        assert_eq!(result, 518);
    }

    #[test]
    fn test_result_sample() {
        let result = get_overlapping_pair_count(TEST_PATH);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_result_input() {
        let result = get_overlapping_pair_count(INPUT_PATH);
        assert_eq!(result, 909);
    }
}
