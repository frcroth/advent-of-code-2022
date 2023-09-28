use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn get_minimal_step_count(
    start: (isize, isize),
    stop: (isize, isize),
    mut step_count: isize,
    height: isize,
    width: isize,
    grid: Vec<Vec<char>>,
) -> isize {
    let mut positions = HashSet::new();
    positions.insert(start);

    loop {
        let mut next_positions = HashSet::new();
        for &(row, col) in &positions {
            for (x, y) in [(row, col), (row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)].iter() {
                if (*x, *y) == stop {
                    return step_count;
                }
                if 0 <= *x
                    && *x < height
                    && 0 <= *y
                    && *y < width
                    && grid[*x as usize][(y - step_count).rem_euclid(width) as usize] != '>'
                    && grid[*x as usize][(y + step_count).rem_euclid(width) as usize] != '<'
                    && grid[(x - step_count).rem_euclid(height) as usize][*y as usize] != 'v'
                    && grid[(x + step_count).rem_euclid(height) as usize][*y as usize] != '^'
                {
                    next_positions.insert((*x, *y));
                }
            }
        }
        positions = next_positions;
        if positions.is_empty() {
            positions.insert(start);
        }
        step_count += 1;
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .skip(1)
        .take(input.lines().count() - 2)
        .map(|line| {
            line.chars()
                .skip(1)
                .take(line.chars().count() - 2)
                .collect()
        })
        .collect()
}

fn part_1(input: &str) -> isize {
    let grid: Vec<Vec<char>> = parse_grid(input);

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let start = (-1, 0);
    let stop = (height, width - 1);

    get_minimal_step_count(start, stop, 1, height, width, grid)
}

fn part_2(input: &str) -> isize {
    let grid: Vec<Vec<char>> = parse_grid(input);

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let start = (-1, 0);
    let stop = (height, width - 1);

    let first_trip = get_minimal_step_count(start, stop, 1, height, width, grid.clone());
    let trip_back = get_minimal_step_count(stop, start, first_trip, height, width, grid.clone());
    get_minimal_step_count(start, stop, trip_back, height, width, grid)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(18, part_1(input));
    assert_eq!(54, part_2(input));
}
