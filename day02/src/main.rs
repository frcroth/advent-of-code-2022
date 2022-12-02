use crate::RockPaperScissors::{Paper, Rock, Scissors};
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(parse_strategy_guide(input)));
    println!("{}", part_2(parse_strategy_guide(input)));
}

fn parse_strategy_guide(input: &str) -> Vec<(char, char)> {
    let re = Regex::new(r"([A-Z]) ([A-Z])").unwrap();
    input
        .split('\n')
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let opponent = caps
                .get(1)
                .map_or('F', |m| m.as_str().chars().next().unwrap());
            let own = caps
                .get(2)
                .map_or('F', |m| m.as_str().chars().next().unwrap());
            (opponent, own)
        })
        .collect::<Vec<(char, char)>>()
}

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

fn char_to_rps(c: char) -> RockPaperScissors {
    match c {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        'X' => Rock,
        'Y' => Paper,
        'Z' => Scissors,
        _ => {
            panic!()
        }
    }
}

fn rps_to_score(rps: RockPaperScissors) -> u32 {
    match rps {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn char_to_score(c: char) -> u32 {
    match c {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn char_to_score_p2(c: char) -> u32 {
    match c {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    }
}

fn score_from_rps_comparison(a: RockPaperScissors, b: RockPaperScissors) -> u32 {
    match a {
        Rock => match b {
            Rock => 3,
            Paper => 0,
            Scissors => 6,
        },
        Paper => match b {
            Rock => 6,
            Paper => 3,
            Scissors => 0,
        },
        Scissors => match b {
            Rock => 0,
            Paper => 6,
            Scissors => 3,
        },
    }
}

fn get_rps_to_achieve_result(opponent: RockPaperScissors, result: char) -> RockPaperScissors {
    match opponent {
        Rock => match result {
            'X' => Scissors,
            'Y' => Rock,
            'Z' => Paper,
            _ => {
                panic!()
            }
        },
        Paper => match result {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => {
                panic!()
            }
        },
        Scissors => match result {
            'X' => Paper,
            'Y' => Scissors,
            'Z' => Rock,
            _ => {
                panic!()
            }
        },
    }
}

fn get_score_of_round(round: (char, char)) -> u32 {
    let (opponent, own) = round;
    let mut score = 0;
    score += char_to_score(own);
    score += score_from_rps_comparison(char_to_rps(own), char_to_rps(opponent));
    score
}

fn get_score_of_round_p2(round: (char, char)) -> u32 {
    let (opponent, desired_result) = round;
    let my_rps = get_rps_to_achieve_result(char_to_rps(opponent), desired_result);
    char_to_score_p2(desired_result) + rps_to_score(my_rps)
}

fn part_1(strategy_guide: Vec<(char, char)>) -> u32 {
    strategy_guide.iter().map(|i| get_score_of_round(*i)).sum()
}

fn part_2(strategy_guide: Vec<(char, char)>) -> u32 {
    strategy_guide
        .iter()
        .map(|i| get_score_of_round_p2(*i))
        .sum()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(15, part_1(parse_strategy_guide(input)));
    assert_eq!(12, part_2(parse_strategy_guide(input)));
}
