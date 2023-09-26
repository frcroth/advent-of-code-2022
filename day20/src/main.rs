fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> isize {
    let numbers = input
        .split('\n')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    decrypt(numbers, 1)
}

fn decrypt(numbers: Vec<isize>, rounds: usize) -> isize {
    let mut numbers: Vec<(usize, isize)> =
        numbers.iter().enumerate().map(|(i, v)| (i, *v)).collect();

    for _ in 0..rounds {
        for original_index in 0..numbers.len() {
            let current_index = numbers.iter().position(|&x| x.0 == original_index).unwrap();
            let number_to_move = numbers[current_index].1;

            let new_index = (current_index as isize + number_to_move)
                .rem_euclid(numbers.len() as isize - 1) as usize;

            let removed_number = numbers.remove(current_index);
            numbers.insert(new_index as usize, removed_number);
        }
    }
    find_coordinates(numbers)
}

fn find_coordinates(numbers: Vec<(usize, isize)>) -> isize {
    let zero_position = numbers.iter().position(|&x| x.1 == 0).unwrap();
    numbers[(zero_position + 1000) % numbers.len()].1
        + numbers[(zero_position + 2000) % numbers.len()].1
        + numbers[(zero_position + 3000) % numbers.len()].1
}

fn part_2(input: &str) -> isize {
    let numbers = input
        .split('\n')
        .map(|s| s.parse::<isize>().unwrap() * 811589153)
        .collect::<Vec<isize>>();
    decrypt(numbers, 10)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(3, part_1(input));
    assert_eq!(1623178306, part_2(input));
}
