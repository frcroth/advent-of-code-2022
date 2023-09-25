use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_voxel_coords(input: &str) -> Vec<(isize, isize, isize)> {
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    input
        .split('\n')
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.get(1)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
                caps.get(2)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
                caps.get(3)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
            )
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let coords = parse_voxel_coords(input);
    let mut voxel_map = HashSet::new();
    for coord in coords.iter() {
        voxel_map.insert(coord);
    }
    count_sides(&voxel_map, &HashSet::new())
}

fn count_sides(
    voxels: &HashSet<&(isize, isize, isize)>,
    exclusion_set: &HashSet<(isize, isize, isize)>,
) -> usize {
    let mut side_count = 0;
    for coord in voxels.iter() {
        let neighbors = get_neighbors(**coord);

        for neighbor in neighbors.iter() {
            if !voxels.contains(neighbor) && !exclusion_set.contains(neighbor) {
                side_count += 1;
            }
        }
    }
    side_count
}

fn get_neighbors(coordinate: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let offsets = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let mut neighbors = Vec::with_capacity(6);
    for &(dx, dy, dz) in offsets.iter() {
        let neighbor_x = coordinate.0 + dx;
        let neighbor_y = coordinate.1 + dy;
        let neighbor_z = coordinate.2 + dz;
        neighbors.push((neighbor_x, neighbor_y, neighbor_z));
    }
    neighbors
}

fn part_2(input: &str) -> usize {
    let coords = parse_voxel_coords(input);
    let mut input_voxels = HashSet::new();
    for coord in coords.iter() {
        input_voxels.insert(coord);
    }
    let mut max_coords = coords[0];
    let mut min_coords = coords[0];

    for &(x, y, z) in coords.iter() {
        max_coords.0 = max_coords.0.max(x);
        max_coords.1 = max_coords.1.max(y);
        max_coords.2 = max_coords.2.max(z);

        min_coords.0 = min_coords.0.min(x);
        min_coords.1 = min_coords.1.min(y);
        min_coords.2 = min_coords.2.min(z);
    }

    // Add padding to voxels so we know voxels that are connected to the outside
    max_coords.0 += 1;
    max_coords.1 += 1;
    max_coords.2 += 1;

    min_coords.0 -= 1;
    min_coords.1 -= 1;
    min_coords.2 -= 1;

    let mut visited = HashSet::new();
    let mut visit_queue = Vec::new();
    let mut connected_parts = Vec::new();

    for x in min_coords.0..max_coords.0 {
        for y in min_coords.1..max_coords.1 {
            for z in min_coords.2..max_coords.2 {
                let voxel = (x, y, z);
                visit_queue.push(voxel);
            }
        }
    }

    while !visit_queue.is_empty() {
        let current_voxel = visit_queue.pop().unwrap();
        if visited.contains(&current_voxel) {
            continue;
        }
        let connected_voxels =
            get_all_voxels_accessible_from(current_voxel, &(min_coords, max_coords), &input_voxels);
        connected_parts.push(connected_voxels.clone());
        visited.extend(connected_voxels);
    }

    // Find enclosed connected parts by checking if they are connected to the outside
    let mut enclosed_voxels = HashSet::new();
    for part in connected_parts.iter() {
        let mut is_enclosed = true;
        for voxel in part.iter() {
            if voxel.0 == max_coords.0
                || voxel.0 == min_coords.0
                || voxel.1 == max_coords.1
                || voxel.1 == min_coords.1
                || voxel.2 == max_coords.2
                || voxel.2 == min_coords.2
            {
                is_enclosed = false;
                break;
            }
            if !is_enclosed {
                break;
            }
        }
        if is_enclosed {
            enclosed_voxels.extend(part);
        }
    }

    count_sides(&input_voxels, &enclosed_voxels)
}

fn get_all_voxels_accessible_from(
    voxel: (isize, isize, isize),
    bounds: &((isize, isize, isize), (isize, isize, isize)),
    exlusion_set: &HashSet<&(isize, isize, isize)>,
) -> HashSet<(isize, isize, isize)> {
    let mut visited = HashSet::new();
    let mut visit_queue = vec![voxel];

    while !visit_queue.is_empty() {
        let current_voxel = visit_queue.pop().unwrap();
        if current_voxel.0 < bounds.0 .0
            || current_voxel.0 > bounds.1 .0
            || current_voxel.1 < bounds.0 .1
            || current_voxel.1 > bounds.1 .1
            || current_voxel.2 < bounds.0 .2
            || current_voxel.2 > bounds.1 .2
        {
            continue;
        }
        if exlusion_set.contains(&current_voxel) {
            continue;
        }
        if visited.contains(&current_voxel) {
            continue;
        }
        visited.insert(current_voxel);
        visit_queue.append(&mut get_neighbors(current_voxel));
    }
    visited
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(64, part_1(input));
    assert_eq!(58, part_2(input));
}
