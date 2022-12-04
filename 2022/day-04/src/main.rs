use core::ops::Range;
use std::env;
use std::fs;

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
    get_count(path, &contains)
}

fn get_overlapping_pair_count(path: &str) -> usize {
    get_count(path, &overlaps)
}

fn get_count(path: &str, f: &dyn Fn(&(Range<u32>, Range<u32>)) -> bool) -> usize {
    let content = fs::read_to_string(path).expect("File should exist");
    let iter = content
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(parse_range_pair)
        .filter(f)
        .collect::<Vec<(Range<u32>, Range<u32>)>>();

    iter.len()
}

fn contains(ranges: &(Range<u32>, Range<u32>)) -> bool {
    (ranges.0.start <= ranges.1.start && ranges.0.end >= ranges.1.end)
        || (ranges.0.start >= ranges.1.start && ranges.0.end <= ranges.1.end)
}

fn overlaps(ranges: &(Range<u32>, Range<u32>)) -> bool {
    (ranges.0.end > ranges.1.start && ranges.0.start <= ranges.1.start)
        || (ranges.1.end > ranges.0.start && ranges.1.start <= ranges.0.start)
}

fn parse_range_pair(input: &str) -> (Range<u32>, Range<u32>) {
    let mut iter = input.split(",").map(parse_str_to_range);
    if let Some(first) = iter.next() {
        if let Some(second) = iter.next() {
            return (first, second);
        }
    }

    panic!("Failed to parse {:?}", input);
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
