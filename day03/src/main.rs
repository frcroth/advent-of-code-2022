use std::collections::hash_map::RandomState;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(parse_rucksack_into_compartments(input)));
    println!("{}", part_2(parse_elf_groups(input)));
}

fn parse_rucksack_into_compartments(input: &str) -> Vec<(String, String)> {
    input
        .split('\n')
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(a, b)| (String::from(a), String::from(b)))
        .collect()
}

fn get_common_letters(rucksack: Vec<(String, String)>) -> Vec<char> {
    rucksack
        .iter()
        .map(|(c1, c2)| {
            let s1: HashSet<char, RandomState> = c1.chars().into_iter().collect();
            let s2: HashSet<char, _> = c2.chars().into_iter().collect();
            s1.intersection(&s2).copied().collect::<Vec<char>>()[0]
        })
        .collect()
}

fn get_score_for_char(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn part_1(rucksack: Vec<(String, String)>) -> u32 {
    get_common_letters(rucksack)
        .into_iter()
        .map(get_score_for_char)
        .sum()
}

fn parse_elf_groups(input: &str) -> Vec<(String, String, String)> {
    input
        .split('\n')
        .into_iter()
        .collect::<Vec<&str>>()
        .as_slice()
        .chunks(3)
        .map(|c| (c[0], c[1], c[2]))
        .map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string()))
        .collect()
}

fn get_common_letter_in_group(group: (String, String, String)) -> char {
    let (c1, c2, c3) = group;
    let s1: HashSet<char, RandomState> = c1.chars().into_iter().collect();
    let s2: HashSet<char, RandomState> = c2.chars().into_iter().collect();
    let s3: HashSet<char, RandomState> = c3.chars().into_iter().collect();
    s1.intersection(&s2)
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&s3)
        .copied()
        .collect::<Vec<char>>()[0]
}

fn part_2(groups: Vec<(String, String, String)>) -> u32 {
    groups
        .into_iter()
        .map(get_common_letter_in_group)
        .map(get_score_for_char)
        .sum()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(157, part_1(parse_rucksack_into_compartments(input)));
    assert_eq!(70, part_2(parse_elf_groups(input)));
}
