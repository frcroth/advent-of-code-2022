use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_int(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }

    fn from_offset(offset: (isize, isize)) -> Direction {
        match offset {
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            _ => panic!("Invalid offset"),
        }
    }

    fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    MoveForward(usize),
}

#[derive(PartialEq)]
enum Tile {
    Open,
    Wall,
    DoesNotExist,
}

#[allow(clippy::type_complexity)]
fn parse_input(
    input: &str,
) -> (
    HashMap<(isize, isize), Tile>,
    Vec<Instruction>,
    (isize, isize),
    (usize, usize),
) {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut grid = HashMap::new();
    let mut start_tile = (-1, -1);
    let mut max = (0, input_parts[0].lines().count() - 1);
    for (y, line) in input_parts[0].lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                _ => Tile::DoesNotExist,
            };
            if y == 0 && tile == Tile::Open && start_tile == (-1, -1) {
                start_tile = (x as isize, y as isize);
            }
            if x > max.0 {
                max.0 = x;
            }
            grid.insert((x as isize, y as isize), tile);
        }
    }
    let mut instructions = vec![];
    let mut iterator = input_parts[1].chars();
    while let Some(c) = iterator.next() {
        if c.is_ascii_digit() {
            let mut number = String::new();
            number.push(c);
            for c in iterator.by_ref() {
                if c.is_ascii_digit() {
                    number.push(c);
                } else {
                    instructions.push(Instruction::MoveForward(number.parse::<usize>().unwrap()));
                    number = String::new();
                    if c == 'R' {
                        instructions.push(Instruction::TurnRight);
                    }
                    if c == 'L' {
                        instructions.push(Instruction::TurnLeft);
                    }
                }
            }
            instructions.push(Instruction::MoveForward(number.parse::<usize>().unwrap()));
        }
    }
    (grid, instructions, start_tile, max)
}

fn perform_instruction(
    instruction: Instruction,
    position: (isize, isize),
    direction: Direction,
    grid: &HashMap<(isize, isize), Tile>,
    max: (usize, usize),
    fold_cube: bool,
) -> ((isize, isize), Direction) {
    match instruction {
        Instruction::TurnLeft => match direction {
            Direction::Up => ((position.0, position.1), Direction::Left),
            Direction::Down => ((position.0, position.1), Direction::Right),
            Direction::Left => ((position.0, position.1), Direction::Down),
            Direction::Right => ((position.0, position.1), Direction::Up),
        },
        Instruction::TurnRight => match direction {
            Direction::Up => ((position.0, position.1), Direction::Right),
            Direction::Down => ((position.0, position.1), Direction::Left),
            Direction::Left => ((position.0, position.1), Direction::Up),
            Direction::Right => ((position.0, position.1), Direction::Down),
        },
        Instruction::MoveForward(distance) => match direction {
            Direction::Up => {
                perform_move_forward((0, -1), position, distance, max, grid, fold_cube)
            }
            Direction::Down => {
                perform_move_forward((0, 1), position, distance, max, grid, fold_cube)
            }
            Direction::Left => {
                perform_move_forward((-1, 0), position, distance, max, grid, fold_cube)
            }
            Direction::Right => {
                perform_move_forward((1, 0), position, distance, max, grid, fold_cube)
            }
        },
    }
}

fn perform_move_forward(
    mut offset: (isize, isize),
    initial_position: (isize, isize),
    steps: usize,
    max: (usize, usize),
    grid: &HashMap<(isize, isize), Tile>,
    fold_cube: bool,
) -> ((isize, isize), Direction) {
    let mut new_position = initial_position;
    #[allow(unused_assignments)]
    let mut previous_position = initial_position;
    let mut possible_new_direction = None;
    for _ in 0..steps {
        previous_position = new_position;
        new_position.0 += offset.0;
        new_position.1 += offset.1;

        if let Some(Tile::DoesNotExist) = grid.get(&(new_position.0, new_position.1)) {
            if fold_cube {
                let (np, new_direction) = find_wrap_around_tile_with_cube(new_position, offset);
                new_position = np;
                possible_new_direction = Some(new_direction);
            } else {
                new_position = find_wrap_around_tile(previous_position, offset, max, grid);
            }
        }
        if grid.get(&(new_position.0, new_position.1)).is_none() {
            if fold_cube {
                let (np, new_direction) = find_wrap_around_tile_with_cube(new_position, offset);
                new_position = np;
                possible_new_direction = Some(new_direction);
            } else {
                new_position = find_wrap_around_tile(previous_position, offset, max, grid);
            }
        }
        if let Some(Tile::Wall) = grid.get(&(new_position.0, new_position.1)) {
            new_position = previous_position;
            break;
        }
        if let Some(dir) = possible_new_direction {
            offset = dir.as_offset();
            possible_new_direction = None;
        }
    }
    (new_position, Direction::from_offset(offset))
}

fn find_wrap_around_tile(
    position: (isize, isize),
    offset: (isize, isize),
    max: (usize, usize),
    grid: &HashMap<(isize, isize), Tile>,
) -> (isize, isize) {
    let initial_position = match offset {
        (1, 0) => (0, position.1),
        (-1, 0) => (max.0 as isize, position.1),
        (0, 1) => (position.0, 0),
        (0, -1) => (position.0, max.1 as isize),
        _ => panic!("Invalid offset"),
    };
    let mut new_position = initial_position;
    while let Some(Tile::DoesNotExist) = grid.get(&(new_position.0, new_position.1)) {
        new_position.0 += offset.0;
        new_position.1 += offset.1;
    }
    while grid.get(&(new_position.0, new_position.1)).is_none() {
        new_position.0 += offset.0;
        new_position.1 += offset.1;
    }
    new_position
}

fn find_wrap_around_tile_with_cube(
    position: (isize, isize),
    offset: (isize, isize),
) -> ((isize, isize), Direction) {
    let (x, y) = position;

    let direction = Direction::from_offset(offset);
    if y == -1 && (50..=99).contains(&x) {
        return ((0, x + 100), Direction::Right);
    }
    if y == -1 && (100..=149).contains(&x) {
        return ((x - 100, 199), Direction::Up);
    }
    if (0..=49).contains(&y) && x == 150 {
        return ((99, 149 - y), Direction::Left);
    }
    if (100..=149).contains(&x) && y == 50 && direction == Direction::Down {
        return ((99, x - 50), Direction::Left);
    }
    if x == 100 && (50..=99).contains(&y) && direction == Direction::Right {
        return ((y + 50, 49), Direction::Up);
    }
    if x == 100 && (100..=150).contains(&y) {
        return ((149, 149 - y), Direction::Left);
    }
    if (50..=99).contains(&x) && y == 150 && direction == Direction::Down {
        return ((49, x + 100), Direction::Left);
    }
    if x == 50 && (150..=199).contains(&y) && direction == Direction::Right {
        return ((y - 100, 149), Direction::Up);
    }
    if (0..=49).contains(&x) && y == 200 {
        return ((x + 100, 0), Direction::Down);
    }
    if x == -1 && (150..=199).contains(&y) {
        return ((y - 100, 0), Direction::Down);
    }
    if x == -1 && (100..=149).contains(&y) {
        return ((50, 149 - y), Direction::Right);
    }
    if (0..=49).contains(&x) && y == 99 && direction == Direction::Up {
        return ((50, x + 50), Direction::Right);
    }
    if x == 49 && (50..=99).contains(&y) && direction == Direction::Left {
        return ((y - 50, 100), Direction::Down);
    }
    if x == 49 && (0..=49).contains(&y) {
        return ((0, 149 - y), Direction::Right);
    }
    unreachable!(
        "Invalid position: {:?}, direction {:?}",
        position, direction
    );
}

#[allow(dead_code)]
fn print_grid(
    grid: &HashMap<(isize, isize), Tile>,
    max: (usize, usize),
    current_position: (isize, isize),
    current_direction: Direction,
) {
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if (current_position.0, current_position.1) == (x as isize, y as isize) {
                match current_direction {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                }
                continue;
            }
            if let Some(Tile::Wall) = grid.get(&(x as isize, y as isize)) {
                print!("#");
            } else if let Some(Tile::Open) = grid.get(&(x as isize, y as isize)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for _ in 0..=max.0 {
        print!("-");
    }
    println!();
}

fn calculate_password(position: (isize, isize), direction: Direction) -> isize {
    1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction.as_int() as isize
}

fn part_1(input: &str) -> isize {
    let (grid, instructions, start_tile, max) = parse_input(input);
    let mut position = start_tile;
    let mut direction = Direction::Right;
    for instruction in instructions {
        let (new_position, new_direction) =
            perform_instruction(instruction, position, direction, &grid, max, false);
        position = new_position;
        direction = new_direction;
    }
    calculate_password(position, direction)
}

fn part_2(input: &str) -> isize {
    let (grid, instructions, start_tile, max) = parse_input(input);
    let mut position = start_tile;
    let mut direction = Direction::Right;
    for instruction in instructions {
        let (new_position, new_direction) =
            perform_instruction(instruction, position, direction, &grid, max, true);
        position = new_position;
        direction = new_direction;
    }
    calculate_password(position, direction)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(6032, part_1(input));
    // Part 2 not tested, because it is hard coded
}
