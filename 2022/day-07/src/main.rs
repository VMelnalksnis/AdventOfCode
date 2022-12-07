use std::env;
use std::fs;

const INPUT_PATH: &str = "input.txt";

fn main() {
    let mut args = env::args().skip(1);
    let filepath = args.next().unwrap_or(INPUT_PATH.to_string());

    let totals = get_count(&filepath);
    println!("Count: {:?}", totals);

    let deleted_size = get_deleted_directory_size(&filepath);
    println!("Deleted size: {:?}", deleted_size);
}

fn get_count(path: &str) -> usize {
    let sizes = get_directory_sizes(path);

    sizes
        .iter()
        .map(|dir| {
            sizes
                .iter()
                .filter(|x| x.0.starts_with(&dir.0))
                .map(|x| x.1)
                .sum::<usize>()
        })
        .filter(|x| x <= &100000)
        .sum::<usize>()
}

fn get_deleted_directory_size(path: &str) -> usize {
    let sizes = get_directory_sizes(path);

    let total_size: usize = 70000000;
    let needed_space: usize = 30000000;
    let used_space: usize = sizes.iter().map(|x| x.1).sum();
    let free_space = total_size - used_space;
    let minimum_deleted = needed_space - free_space;

    let mut dirs = sizes
        .iter()
        .map(|dir| {
            sizes
                .iter()
                .filter(|x| x.0.starts_with(&dir.0))
                .map(|x| x.1)
                .sum::<usize>()
        })
        .filter(|x| x >= &minimum_deleted)
        .collect::<Vec<usize>>();

    dirs.sort();

    dirs[0]
}

fn get_directory_sizes(path: &str) -> Vec<(String, usize)> {
    let content = fs::read_to_string(&path).expect("File should exist");
    let lines = content.lines().rev().collect::<Vec<&str>>();
    let x = lines
        .split_inclusive(|line| line.chars().next() == Some('$'))
        .rev()
        .map(|x| x.into_iter().rev().cloned().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut current_dir = "".to_string();
    let mut sizes: Vec<(String, usize)> = vec![];

    for commands in x.into_iter() {
        let mut iter = commands.into_iter();
        let cmd = iter
            .next()
            .expect("Should contain single line")
            .split(" ")
            .collect::<Vec<&str>>();

        match cmd[..] {
            ["$", "ls"] => {
                let size = iter
                    .filter(|line| !line.starts_with("dir"))
                    .map(|line| {
                        line.split(" ")
                            .next()
                            .expect("")
                            .parse::<usize>()
                            .expect("")
                    })
                    .sum::<usize>();

                sizes.push((current_dir.clone(), size));
            }
            ["$", "cd", x] => match x {
                ".." => {
                    let index = current_dir
                        .rfind("/")
                        .expect("should contain directory separator");

                    if index == 0 {
                        current_dir = "/".to_string();
                    } else {
                        current_dir = current_dir[0..index].to_string();
                    }
                }
                new_dir => {
                    if current_dir.ends_with("/") || new_dir.starts_with("/") {
                        current_dir = current_dir + new_dir;
                    } else {
                        current_dir = current_dir + "/" + new_dir;
                    }
                }
            },
            _ => panic!("Unexpected command"),
        }
    }

    sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &str = "sample.txt";

    #[test]
    fn test_sample() {
        let result = get_count(TEST_PATH);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_input() {
        let result = get_count(INPUT_PATH);
        assert_eq!(result, 1453349);
    }

    #[test]
    fn test_second_sample() {
        let result = get_deleted_directory_size(TEST_PATH);
        assert_eq!(result, 24933642);
    }

    #[test]
    fn test_second_input() {
        let result = get_deleted_directory_size(INPUT_PATH);
        assert_eq!(result, 2948823);
    }
}
