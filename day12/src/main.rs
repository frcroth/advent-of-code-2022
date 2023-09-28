use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    let start = ['S'];
    solve(grid, &start)
}

fn part_2(input: &str) -> i32 {
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    let start = ['S', 'a'];
    solve(grid, &start)
}

fn solve(grid: Vec<Vec<char>>, possible_starts: &[char]) -> i32 {
    let mut visit_queue = VecDeque::new();
    let mut visited = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if possible_starts.contains(&grid[i][j]) {
                visit_queue.push_back((i as i32, j as i32, 0, 'a'));
                visited.insert((i as i32, j as i32));
            }
        }
    }

    fn visit(i: i32, j: i32, d: i32, current_elevation: char, grid: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>, visit_queue: &mut VecDeque<(i32, i32, i32, char)>) {
        if !(0 <= i && i < grid.len() as i32) || !(0 <= j && j < grid[i as usize].len() as i32) {
            return;
        }
        if visited.contains(&(i, j)) {
            return;
        }
        let next_elevation = grid[i as usize][j as usize].to_string().replace("E", "z").chars().next().unwrap();
        if next_elevation as u8 > (current_elevation as u8 + 1) {
            return;
        }
        visited.insert((i, j));
        visit_queue.push_back((i, j, d + 1, next_elevation));
    }

    while let Some((i, j, d, a)) = visit_queue.pop_front() {
        if grid[i as usize][j as usize] == 'E' {
            return d;
        }
        visit(i + 1, j, d, a, &grid, &mut visited, &mut visit_queue);
        visit(i - 1, j, d, a, &grid, &mut visited, &mut visit_queue);
        visit(i, j + 1, d, a, &grid, &mut visited, &mut visit_queue);
        visit(i, j - 1, d, a, &grid, &mut visited, &mut visit_queue);
    }

    unreachable!()
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");
    assert_eq!(31, part_1(input));
    assert_eq!(29, part_2(input));
}
