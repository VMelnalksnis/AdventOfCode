use itertools::Itertools;
use std::env;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let count = get_count(&filepath);
    println!("Count: {:?}", count);

    let score = get_scenic_score(&filepath);
    println!("Score: {:?}", score);
}

fn get_count(path: &str) -> usize {
    let content = fs::read_to_string(&path).expect("File should exist");
    let lines = content.lines().collect::<Vec<&str>>();

    let mut trees: Vec<Vec<u8>> = vec![];
    for (y, line) in lines.into_iter().enumerate() {
        trees.push(vec![]);

        for char in line.chars().into_iter() {
            let height = char.to_digit(10).expect("a digit") as u8;
            trees[y].push(height);
        }
    }

    let mut visible: usize = 0;

    for (y, heights) in trees.iter().enumerate() {
        for (x, height) in heights.iter().enumerate() {
            if y == 0 || x == 0 || y == trees.len() || x == heights.len() {
                visible = visible + 1;
                continue;
            }

            let mut left = heights[0..x].iter();
            let mut right = heights[x + 1..heights.len()].iter();
            let mut top = trees.iter().enumerate().filter(|c| c.0 < y).map(|c| c.1[x]);
            let mut bottom = trees.iter().enumerate().filter(|c| c.0 > y).map(|c| c.1[x]);

            if left.all(|h| h < height)
                || right.all(|h| h < height)
                || top.all(|h| h < *height)
                || bottom.all(|h| h < *height)
            {
                visible = visible + 1;
            }
        }
    }

    visible
}

fn get_scenic_score(path: &str) -> usize {
    let content = fs::read_to_string(&path).expect("File should exist");
    let lines = content.lines().collect::<Vec<&str>>();

    let mut trees: Vec<Vec<u8>> = vec![];
    for (y, line) in lines.into_iter().enumerate() {
        trees.push(vec![]);

        for char in line.chars().into_iter() {
            let height = char.to_digit(10).expect("a digit") as u8;
            trees[y].push(height);
        }
    }

    let mut scores: Vec<usize> = vec![];

    for (y, heights) in trees.iter().enumerate() {
        for (x, height) in heights.iter().enumerate() {
            if y == 0 || x == 0 || y == trees.len() - 1 || x == heights.len() - 1 {
                continue;
            }

            let score = calculate_score(x, y, &heights, height, &trees);
            scores.push(score);
        }
    }

    scores.sort();
    scores.into_iter().rev().next().expect("should have values")
}

fn calculate_score(
    x: usize,
    y: usize,
    heights: &Vec<u8>,
    height: &u8,
    trees: &Vec<Vec<u8>>,
) -> usize {
    let mut left_trees_iter = heights[0..x].iter().rev();
    let mut left = left_trees_iter.by_ref().take_while(|h| h < &height).count();
    if left_trees_iter.next().is_some() {
        left = left + 1;
    }

    let mut right_trees_iter = heights[x + 1..heights.len()].iter();
    let mut right = right_trees_iter.take_while_ref(|h| h < &height).count();
    if right_trees_iter.next().is_some() {
        right = right + 1;
    }

    let mut top_trees_iter = trees
        .iter()
        .enumerate()
        .filter(|c| c.0 < y)
        .map(|c| c.1[x])
        .rev();

    let mut top = top_trees_iter.by_ref().take_while(|h| h < height).count();
    if top_trees_iter.next().is_some() {
        top = top + 1;
    }

    let mut bottom_trees_iter = trees.iter().enumerate().filter(|c| c.0 > y).map(|c| c.1[x]);
    let mut bottom = bottom_trees_iter
        .by_ref()
        .take_while(|h| h < height)
        .count();
    if bottom_trees_iter.next().is_some() {
        bottom = bottom + 1;
    }

    (left) * (right) * (top) * (bottom)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = get_count(TEST_PATH);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_input() {
        let result = get_count(INPUT_PATH);
        assert_eq!(result, 1849);
    }

    #[test]
    fn test_second_sample() {
        let result = get_scenic_score(TEST_PATH);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_second_input() {
        let result = get_scenic_score(INPUT_PATH);
        assert_eq!(result, 201600);
    }
}
