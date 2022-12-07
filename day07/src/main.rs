use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

// https://stackoverflow.com/questions/68837763/how-to-iterate-prefixes-or-suffixes-of-vec-or-slice-in-rust
pub fn prefixes_asc<T>(slice: &[T]) -> impl Iterator<Item = &[T]> + DoubleEndedIterator {
    (0..=slice.len()).map(move |len| &slice[..len])
}

fn part_1(input: &str) -> u32 {
    get_directory_sizes(input)
        .iter()
        .filter(|(_, sum)| **sum < 100000)
        .map(|(_, sum)| sum)
        .sum()
}

fn part_2(input: &str) -> u32 {
    let total_space = 70000000;
    let needed_space = 30000000;
    let directory_sizes = get_directory_sizes(input);
    let available_space = total_space - *directory_sizes.get("/").unwrap();
    let space_to_delete = needed_space - available_space;
    let mut possible_directory_sizes: Vec<u32> = directory_sizes
        .iter()
        .filter(|(_, s)| **s > space_to_delete)
        .map(|(_, s)| *s)
        .collect();
    possible_directory_sizes.sort_unstable();
    possible_directory_sizes[0]
}

fn get_directory_sizes(input: &str) -> HashMap<String, u32> {
    let mut directory_sizes = HashMap::new();
    let mut directory_path: Vec<&str> = vec![];
    let commands = input.split('\n').collect::<Vec<&str>>();
    let cd_regex = Regex::new(r"\$ cd ([a-z]+|/)").unwrap();
    for c in commands {
        if cd_regex.is_match(c) {
            let caps = cd_regex.captures(c).unwrap();
            directory_path.push(caps.get(1).unwrap().as_str());
        } else if c == "$ cd .." {
            directory_path.pop();
        } else if c.starts_with("dir") || c.starts_with("$ ls") {
            // Ignore, assuming that ls is only done once per directory
        } else {
            let size = c.split(' ').next().unwrap().parse::<u32>().unwrap();
            for prefix in prefixes_asc(directory_path.as_slice()) {
                let full_directory = prefix.join("/");
                *directory_sizes.entry(full_directory).or_insert(0) += size;
            }
        }
    }
    directory_sizes
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(95437, part_1(input));
    assert_eq!(24933642, part_2(input));
}
