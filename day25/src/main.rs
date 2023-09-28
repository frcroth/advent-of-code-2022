fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
}

fn part_1(input: &str) -> String {
    to_snafu(input.lines().map(|l| from_snafu(l)).sum())
}

fn from_snafu(snafu: &str) -> usize {
    let mut sum = 0;
    for (i, c) in snafu.chars().rev().enumerate() {
        let factor: isize = (5isize).pow(i as u32);
        sum += match c {
            '0' => 0,
            '1' => 1 * factor,
            '2' => 2 * factor,
            '=' => -2 * factor,
            '-' => -1 * factor,
            _ => panic!("Invalid character in snafu: {}", c),
        }
    }
    sum as usize
}

fn to_snafu(number: usize) -> String {
    let mut number = number as isize;
    let mut snafu = String::new();
    while number != 0 {
        let a = (number + 2) % 5;
        snafu.push(match a {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!()
        });
        if a < 2 {
            number += 5;
        }
        number /= 5;
    }
    snafu.chars().rev().collect()
}

#[test]
fn test_snafu_conversion() {

    assert_eq!("1", to_snafu(1));
    assert_eq!("2", to_snafu(2));
    assert_eq!("1=", to_snafu(3));
    assert_eq!("1-", to_snafu(4));
    assert_eq!("10", to_snafu(5));
    assert_eq!("11", to_snafu(6));
    assert_eq!("12", to_snafu(7));
    assert_eq!("2=", to_snafu(8));
    assert_eq!("2-", to_snafu(9));
    assert_eq!("20", to_snafu(10));
    assert_eq!("1=0", to_snafu(15));
    assert_eq!("1-0", to_snafu(20));
    assert_eq!("1=11-2", to_snafu(2022));
    assert_eq!("1-0---0", to_snafu(12345));
    assert_eq!("1121-1110-1=0", to_snafu(314159265));

    assert_eq!(1, from_snafu("1"));
    assert_eq!(2, from_snafu("2"));
    assert_eq!(3, from_snafu("1="));
    assert_eq!(4, from_snafu("1-"));
    assert_eq!(5, from_snafu("10"));
    assert_eq!(6, from_snafu("11"));
    assert_eq!(7, from_snafu("12"));
    assert_eq!(8, from_snafu("2="));
    assert_eq!(9, from_snafu("2-"));
    assert_eq!(10, from_snafu("20"));
    assert_eq!(15, from_snafu("1=0"));
    assert_eq!(20, from_snafu("1-0"));
    assert_eq!(2022, from_snafu("1=11-2"));
    assert_eq!(12345, from_snafu("1-0---0"));
    assert_eq!(314159265, from_snafu("1121-1110-1=0"));
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!("2=-1=0".to_string(), part_1(input));
}