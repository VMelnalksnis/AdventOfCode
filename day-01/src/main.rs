use std::env;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or("input.txt".to_string());
    let count = args
        .next()
        .unwrap_or(3.to_string())
        .parse::<usize>()
        .unwrap();

    find_largest(&filepath);
    find_n_largest(&filepath, count);
}

fn find_largest(path: &str) -> u32 {
    let x = get_totals(path);
    let max = x.iter().max().unwrap();
    println!("Largest sum: {}", max);
    *max
}

fn find_n_largest(path: &str, n: usize) -> u32 {
    let mut x = get_totals(path);
    x.sort();
    let total = x.iter().rev().take(n).sum::<u32>();
    println!("Sum of largest {0} sums: {1}", n, total);
    total
}

fn get_totals(path: &str) -> Vec<u32> {
    let content = fs::read_to_string(path).expect("foo");
    let groups: Vec<String> = content
        .split("\r\n")
        .map(|x| x.trim().replace("\u{feff}", ""))
        .collect();

    let numbers: Vec<Result<u32, core::num::ParseIntError>> =
        groups.iter().map(|x| x.parse::<u32>()).collect();

    let x = numbers
        .split(|x| x.is_err())
        .map(|x| x.iter().map(|v| v.as_ref().unwrap()).collect::<Vec<&u32>>())
        .filter(|x| !x.is_empty())
        .map(|x| x.into_iter().sum())
        .collect::<Vec<u32>>();

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = find_largest(TEST_PATH);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_sample_n() {
        let result = find_n_largest(TEST_PATH, 3);
        assert_eq!(result, 45000);
    }
}
