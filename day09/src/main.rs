use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn is_touching(pos1: (i32, i32), pos2: (i32, i32)) -> bool {
    i32::abs(pos1.0 - pos2.0) < 2 && i32::abs(pos1.1 - pos2.1) < 2
}

fn clamp(val: i32, min: i32, max: i32) -> i32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn get_new_tail_position(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head != tail && !is_touching(head, tail) {
        return (
            tail.0 + clamp(head.0 - tail.0, -1, 1),
            tail.1 + clamp(head.1 - tail.1, -1, 1),
        );
    }
    tail
}

fn get_new_head_position(head: (i32, i32), command: &str) -> (i32, i32) {
    match command {
        "R" => (head.0 + 1, head.1),
        "L" => (head.0 - 1, head.1),
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        _ => head,
    }
}

fn part_1(input: &str) -> usize {
    let mut positions = HashSet::new();
    let r = Regex::new(r"([RULD]) ([0-9]+)").unwrap();
    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);
    for line in input.split('\n') {
        let caps = r.captures(line).unwrap();
        let command = caps.get(1).unwrap().as_str();
        let steps = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        for _ in 0..steps {
            h_pos = get_new_head_position(h_pos, command);
            t_pos = get_new_tail_position(h_pos, t_pos);
            positions.insert(t_pos);
        }
    }

    positions.len()
}

fn part_2(input: &str) -> usize {
    let mut positions = HashSet::new();
    let r = Regex::new(r"([RULD]) ([0-9]+)").unwrap();
    let mut h_pos = (0, 0);
    let mut tail_knots = vec![];
    tail_knots.resize(9, (0, 0));
    for line in input.split('\n') {
        let caps = r.captures(line).unwrap();
        let command = caps.get(1).unwrap().as_str();
        let steps = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        for _ in 0..steps {
            h_pos = get_new_head_position(h_pos, command);
            tail_knots[0] = get_new_tail_position(h_pos, tail_knots[0]);
            for i in 0..8 {
                tail_knots[i + 1] = get_new_tail_position(tail_knots[i], tail_knots[i + 1]);
            }
            positions.insert(tail_knots[8]);
        }
    }

    positions.len()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(13, part_1(input));
    assert_eq!(1, part_2(input));
}
