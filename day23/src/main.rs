use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroveTile {
    Elf,
    Empty,
}

enum Direction {
    North,
    South,
    West,
    East,
}

fn parse_grove(input: &str) -> HashMap<(isize, isize), GroveTile> {
    let mut grove = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grove.insert(
                (x as isize, y as isize),
                match c {
                    '#' => GroveTile::Elf,
                    '.' => GroveTile::Empty,
                    _ => panic!("Unknown tile type"),
                },
            );
        })
    });
    grove
}

fn no_elf_at_position(
    position: &(isize, isize),
    grove: &HashMap<(isize, isize), GroveTile>,
) -> bool {
    !matches!(grove.get(position), Some(GroveTile::Elf))
}

fn get_move_proposition(
    current_position: &(isize, isize),
    direction: &Direction,
    grove: &HashMap<(isize, isize), GroveTile>,
) -> Option<(isize, isize)> {
    // If no elves are in the 8 tiles around the elf, do not move at all
    let neighbors = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    if !neighbors
        .map(|n| no_elf_at_position(&(current_position.0 + n.0, current_position.1 + n.1), grove))
        .iter()
        .any(|b| !*b)
    {
        return None;
    }

    match direction {
        Direction::North => {
            if no_elf_at_position(&(current_position.0, current_position.1 - 1), grove)
                && no_elf_at_position(&(current_position.0 - 1, current_position.1 - 1), grove)
                && no_elf_at_position(&(current_position.0 + 1, current_position.1 - 1), grove)
            {
                Some((current_position.0, current_position.1 - 1))
            } else {
                None
            }
        }
        Direction::South => {
            if no_elf_at_position(&(current_position.0, current_position.1 + 1), grove)
                && no_elf_at_position(&(current_position.0 - 1, current_position.1 + 1), grove)
                && no_elf_at_position(&(current_position.0 + 1, current_position.1 + 1), grove)
            {
                Some((current_position.0, current_position.1 + 1))
            } else {
                None
            }
        }
        Direction::West => {
            if no_elf_at_position(&(current_position.0 - 1, current_position.1), grove)
                && no_elf_at_position(&(current_position.0 - 1, current_position.1 - 1), grove)
                && no_elf_at_position(&(current_position.0 - 1, current_position.1 + 1), grove)
            {
                Some((current_position.0 - 1, current_position.1))
            } else {
                None
            }
        }
        Direction::East => {
            if no_elf_at_position(&(current_position.0 + 1, current_position.1), grove)
                && no_elf_at_position(&(current_position.0 + 1, current_position.1 - 1), grove)
                && no_elf_at_position(&(current_position.0 + 1, current_position.1 + 1), grove)
            {
                Some((current_position.0 + 1, current_position.1))
            } else {
                None
            }
        }
    }
}

#[allow(dead_code)]
fn print_grove(grove: &HashMap<(isize, isize), GroveTile>) {
    for y in -2..=10 {
        for x in -3..=10 {
            if let Some(tile) = grove.get(&(x, y)) {
                match tile {
                    GroveTile::Elf => print!("#"),
                    _ => print!("."),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_1(input: &str) -> usize {
    let mut grove = parse_grove(input);

    let mut current_direction_index = 0;
    for _ in 0..10 {
        simulate_round(&mut grove, current_direction_index);
        current_direction_index = (current_direction_index + 1) % 4;
    }
    count_empty_ground_tiles(&grove)
}

fn simulate_round(
    grove: &mut HashMap<(isize, isize), GroveTile>,
    current_direction_index: usize,
) -> bool {
    let directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let elf_positions: Vec<((isize, isize), GroveTile)> = grove
        .iter()
        .filter(|(_, tile)| matches!(tile, GroveTile::Elf))
        .map(|(position, tile)| (*position, *tile))
        .collect::<Vec<_>>();
    let mut proposed_elf_positions = HashMap::new();
    let mut proposed_elf_positions_original_positions = HashMap::new();
    for elf in elf_positions {
        for direction in directions
            .iter()
            .cycle()
            .skip(current_direction_index)
            .take(4)
        {
            let move_proposition = get_move_proposition(&elf.0, direction, grove);
            if let Some(move_proposition) = move_proposition {
                if let Some(count) = proposed_elf_positions.get(&move_proposition) {
                    proposed_elf_positions.insert(move_proposition, count + 1);
                } else {
                    proposed_elf_positions.insert(move_proposition, 1);
                }
                proposed_elf_positions_original_positions.insert(move_proposition, elf.0);
                break;
            }
        }
    }
    // Second half

    let mut changed = false;

    for (new_position, count) in proposed_elf_positions {
        if count > 1 {
            continue;
        }
        changed = true;
        let original_position = proposed_elf_positions_original_positions
            .get(&new_position)
            .unwrap();
        grove.insert(new_position, GroveTile::Elf);
        grove.insert(*original_position, GroveTile::Empty);
    }
    changed
}

fn count_empty_ground_tiles(grove: &HashMap<(isize, isize), GroveTile>) -> usize {
    let mut max = (0, 0);
    let mut min = (isize::MAX, isize::MAX);
    for (position, tile) in grove.iter() {
        if tile == &GroveTile::Elf {
            if position.0 > max.0 {
                max.0 = position.0;
            }
            if position.1 > max.1 {
                max.1 = position.1;
            }
            if position.0 < min.0 {
                min.0 = position.0;
            }
            if position.1 < min.1 {
                min.1 = position.1;
            }
        }
    }
    let mut count = 0;
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            let found = grove.get(&(x, y));
            match found {
                Some(GroveTile::Elf) => {}
                _ => count += 1,
            }
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let mut grove = parse_grove(input);

    let mut current_direction_index = 0;
    let mut changed = true;
    let mut i = 0;
    while changed {
        changed = simulate_round(&mut grove, current_direction_index);
        current_direction_index = (current_direction_index + 1) % 4;
        i += 1;
    }
    i
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(110, part_1(input));
    assert_eq!(20, part_2(input));
}
