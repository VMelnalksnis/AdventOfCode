use std::env;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let first = get_first_marker_index(&filepath, 4);
    println!("First: {:?}", first);

    let second = get_first_marker_index(&filepath, 14);
    println!("Second: {:?}", second);
}

fn get_first_marker_index(path: &str, window: usize) -> usize {
    let content = fs::read_to_string(path).expect("File should exist");
    let line = content.lines().next().expect("Should have single line");
    let chars = line.chars().collect::<Vec<char>>();

    for (i, c) in chars.windows(window).enumerate() {
        let mut x = c.iter().cloned().collect::<Vec<char>>();
        x.sort();
        x.dedup();
        if x.len() == window {
            return i + window;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = get_first_marker_index(TEST_PATH, 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_input() {
        let result = get_first_marker_index(INPUT_PATH, 4);
        assert_eq!(result, 1850);
    }

    #[test]
    fn test_second_sample() {
        let result = get_first_marker_index(TEST_PATH, 14);
        assert_eq!(result, 19);
    }

    #[test]
    fn test_second_input() {
        let result = get_first_marker_index(INPUT_PATH, 14);
        assert_eq!(result, 2823);
    }
}
