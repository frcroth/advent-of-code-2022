use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_ranges(input: &str) -> Vec<(u32, u32, u32, u32)> {
    let re = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
    input
        .split('\n')
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.get(1)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
                caps.get(2)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
                caps.get(3)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
                caps.get(4)
                    .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            )
        })
        .collect()
}

fn range_fully_contained(ranges: (u32, u32, u32, u32)) -> bool {
    let (a, b, c, d) = ranges;
    (a <= c && b >= d) || (c <= a && d >= b)
}

fn range_overlaps(ranges: (u32, u32, u32, u32)) -> bool {
    let (a, b, c, d) = ranges;
    (b >= c && a <= c) || (c <= a && d >= a) || range_fully_contained(ranges)
}

fn part_1(input: &str) -> usize {
    parse_ranges(input)
        .into_iter()
        .filter(|(a, b, c, d)| range_fully_contained((*a, *b, *c, *d)))
        .count()
}

fn part_2(input: &str) -> usize {
    parse_ranges(input)
        .into_iter()
        .filter(|(a, b, c, d)| range_overlaps((*a, *b, *c, *d)))
        .count()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(2, part_1(input));
    assert_eq!(4, part_2(input));
}
