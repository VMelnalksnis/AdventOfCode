#![feature(slice_group_by)]

use std::collections::HashSet;
use std::env;
use std::fs;
use std::str;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());
    let sum = get_sum(&filepath);
    let sum_grouped = get_sum_grouped(&filepath);
    println!("Sum: {}", sum);
    println!("Grouped sum: {}", sum_grouped);
}

fn get_sum(path: &str) -> u32 {
    let bytes = fs::read(path).expect("Unable to read file");
    get_input_lines(&bytes)
        .into_iter()
        .map(|x| {
            if x.len() % 2 != 0 {
                panic!("{:?}", x)
            } else {
                x
            }
        })
        .map(|x| {
            let mut iter = x.chunks(x.len() / 2);
            let first: HashSet<u8> = iter.next().expect(" ").iter().cloned().collect();
            let second: HashSet<u8> = iter.next().expect(" ").iter().cloned().collect();
            let mut intersection = first.intersection(&second).cloned();
            match intersection.next() {
                None => panic!("No intersecting elements"),
                Some(element) => match intersection.next() {
                    None => element,
                    Some(_) => panic!("Multiple intersecting elements"),
                },
            }
        })
        .map(map_byte_to_score)
        .sum()
}

fn get_sum_grouped(path: &str) -> u32 {
    let bytes = fs::read(path).expect("Unable to read file");

    get_input_lines(&bytes)
        .into_iter()
        .map(|x| {
            let mut vec = x.iter().cloned().collect::<Vec<u8>>();
            vec.sort();
            vec.dedup();
            vec
        })
        .collect::<Vec<Vec<u8>>>()
        .chunks(3)
        .map(|x| {
            let mut y = x.iter().flatten().cloned().collect::<Vec<u8>>();
            y.sort();
            y.group_by(|a, b| a == b)
                .filter(|x| x.len() == 3)
                .next()
                .expect("")
                .iter()
                .cloned()
                .next()
                .expect("")
        })
        .map(map_byte_to_score)
        .sum()
}

fn get_input_lines(bytes: &Vec<u8>) -> Vec<&[u8]> {
    bytes
        .split(|x| *x == 10u8)
        .filter(|x| match x {
            [a] if *a == 13u8 => false,
            [] => false,
            _ => true,
        })
        .map(|x| match x {
            [.., 13u8] => x.split_last().unwrap().1,
            _ => x,
        })
        .collect()
}

fn map_byte_to_score(byte: u8) -> u32 {
    match byte {
        65..=90 => byte as u32 - 38,
        97..=122 => byte as u32 - 96,
        _ => panic!("unexpected char {:?}", byte),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = get_sum(TEST_PATH);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_input() {
        let result = get_sum(INPUT_PATH);
        assert_eq!(result, 7831);
    }

    #[test]
    fn test_result_sample() {
        let result = get_sum_grouped(TEST_PATH);
        assert_eq!(result, 70);
    }

    #[test]
    fn test_result_input() {
        let result = get_sum_grouped(INPUT_PATH);
        assert_eq!(result, 2683);
    }
}
