use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

#[derive(PartialEq)]
enum Element {
    Sand,
    Stone,
    Air,
}

fn get_coordinates_between_points(a: (i64, i64), b: (i64, i64)) -> Vec<(i64, i64)> {
    let mut vec = vec![];
    let is_vertical = a.0 == b.0;
    if is_vertical {
        let diff = b.1 - a.1;
        if diff > 0 {
            for i in 0..=diff {
                vec.push((a.0, a.1 + i));
            }
        } else {
            for i in diff..=0 {
                vec.push((a.0, a.1 + i));
            }
        }
    } else {
        let diff = b.0 - a.0;
        if diff > 0 {
            for i in 0..=diff {
                vec.push((a.0 + i, a.1));
            }
        } else {
            for i in diff..=0 {
                vec.push((a.0 + i, a.1));
            }
        }
    }
    vec
}

fn get_stones_for_stone_spec(stone_spec: &str) -> Vec<(i64, i64)> {
    let mut stone_positions: Vec<(i64, i64)> = vec![];
    let coordinates = stone_spec.split(" -> ");
    let points = coordinates
        .into_iter()
        .map(|c| {
            let (x_str, y_str) = c.split_once(',').unwrap();
            let x = x_str.parse::<i64>().unwrap();
            let y = y_str.parse::<i64>().unwrap();
            (x, y)
        })
        .collect::<Vec<(i64, i64)>>();
    for next_points in points.windows(2) {
        let point_0 = next_points[0];
        let point_1 = next_points[1];
        stone_positions.append(&mut get_coordinates_between_points(point_0, point_1));
    }
    stone_positions
}

fn parse_lines(input: &str) -> (HashMap<(i64, i64), Element>, i64) {
    let mut map = HashMap::new();
    let mut max_y = 0;
    let split = input.split('\n').collect::<Vec<&str>>();
    for line in split {
        let stones = get_stones_for_stone_spec(line);
        for (x, y) in stones {
            max_y = max_y.max(y);
            map.insert((x, y), Element::Stone);
        }
    }
    (map, max_y)
}

fn drop_sand_p1(map: &HashMap<(i64, i64), Element>, max_y: i64) -> (bool, (i64, i64)) {
    let mut sand_position = (500, 0);

    loop {
        if sand_position.1 > max_y {
            return (true, (0, 0));
        }
        let below = map
            .get(&(sand_position.0, sand_position.1 + 1))
            .unwrap_or(&{ Element::Air });
        let below_left = map
            .get(&(sand_position.0 - 1, sand_position.1 + 1))
            .unwrap_or(&{ Element::Air });
        let below_right = map
            .get(&(sand_position.0 + 1, sand_position.1 + 1))
            .unwrap_or(&{ Element::Air });

        if *below == Element::Air {
            sand_position = (sand_position.0, sand_position.1 + 1);
            continue;
        }
        if *below_left == Element::Air {
            sand_position = (sand_position.0 - 1, sand_position.1 + 1);
            continue;
        }
        if *below_right == Element::Air {
            sand_position = (sand_position.0 + 1, sand_position.1 + 1);
            continue;
        }
        // Can not move
        return (false, sand_position);
    }
}

fn part_1(input: &str) -> i64 {
    let (mut map, max_y) = parse_lines(input);
    let mut steps = 0;
    loop {
        let (fallen_into_void, position_fallen_to) = drop_sand_p1(&map, max_y);
        if fallen_into_void {
            return steps;
        }
        steps += 1;
        map.insert(position_fallen_to, Element::Sand);
    }
}

fn drop_sand_p2(map: &mut HashMap<(i64, i64), Element>, max_y: i64) -> usize {
    let mut sand_position;

    let floor_level = max_y + 2;
    let mut steps = 0;

    loop {
        steps += 1;
        sand_position = (500, 0);
        // Inner loop: Move sand block as far down as possible
        loop {
            if sand_position.1 + 1 == floor_level {
                break;
            }
            let below = map
                .get(&(sand_position.0, sand_position.1 + 1))
                .unwrap_or(&{ Element::Air });
            let below_left = map
                .get(&(sand_position.0 - 1, sand_position.1 + 1))
                .unwrap_or(&{ Element::Air });
            let below_right = map
                .get(&(sand_position.0 + 1, sand_position.1 + 1))
                .unwrap_or(&{ Element::Air });

            if *below == Element::Air {
                sand_position = (sand_position.0, sand_position.1 + 1);
                continue;
            }
            if *below_left == Element::Air {
                sand_position = (sand_position.0 - 1, sand_position.1 + 1);
                continue;
            }
            if *below_right == Element::Air {
                sand_position = (sand_position.0 + 1, sand_position.1 + 1);
                continue;
            }
            break;
        }
        if sand_position == (500, 0) {
            return steps;
        }
        map.insert(sand_position, Element::Sand);
    }
}

fn part_2(input: &str) -> usize {
    let (mut map, max_y) = parse_lines(input);
    drop_sand_p2(&mut map, max_y)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(24, part_1(input));
    assert_eq!(93, part_2(input));
}