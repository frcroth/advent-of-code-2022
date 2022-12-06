extern crate core;

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|(_, slice)| {
            let s: HashSet<char> = (**slice).iter().cloned().collect();
            s.len() == 4
        })
        .unwrap()
        .0 as u32
        + 4
}

fn part_2(input: &str) -> u32 {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
        .find(|(_, slice)| {
            let s: HashSet<char> = (**slice).iter().cloned().collect();
            s.len() == 14
        })
        .unwrap()
        .0 as u32
        + 14
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(7, part_1(input));
    assert_eq!(19, part_2(input));
}
