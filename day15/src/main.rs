use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input, 2000000));
    println!("{}", part_2(input, 4000000));
}

fn parse_sensor_and_beacon_list(input: &str) -> Vec<(i64, i64, i64, i64)> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input
        .split('\n')
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.get(1)
                    .map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
                caps.get(2)
                    .map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
                caps.get(3)
                    .map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
                caps.get(4)
                    .map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
            )
        })
        .collect()
}

fn get_radius(sensor_beacon_tuple: (i64, i64, i64, i64)) -> i64 {
    (sensor_beacon_tuple.0 - sensor_beacon_tuple.2).abs()
        + (sensor_beacon_tuple.1 - sensor_beacon_tuple.3).abs()
}

fn within_sensor_range(sensor_beacon_tuple: (i64, i64, i64, i64), other: (i64, i64)) -> bool {
    let sensor_to_other = get_radius((
        sensor_beacon_tuple.0,
        sensor_beacon_tuple.1,
        other.0,
        other.1,
    ));
    sensor_to_other <= get_radius(sensor_beacon_tuple)
}

fn part_1(input: &str, target_line: i64) -> usize {
    let sensors_and_beacons = parse_sensor_and_beacon_list(input);
    let occupied_positions: HashSet<(i64, i64)> = sensors_and_beacons
        .iter()
        .flat_map(|tuple| [(tuple.0, tuple.1), (tuple.2, tuple.3)])
        .collect();

    let min_x = sensors_and_beacons
        .iter()
        .map(|tuple| tuple.0.min(tuple.2))
        .min()
        .unwrap();
    let max_x = sensors_and_beacons
        .iter()
        .map(|tuple| tuple.1.min(tuple.3))
        .max()
        .unwrap();

    let max_dist = sensors_and_beacons
        .iter()
        .map(|tuple| get_radius(*tuple))
        .max()
        .unwrap();
    let start_x = min_x - max_dist;
    let end_x = max_x + max_dist;

    let mut num_points_in_range = 0;
    for x in start_x..=end_x {
        let position = (x, target_line);
        if occupied_positions.contains(&position) {
            continue;
        }
        if sensors_and_beacons
            .iter()
            .any(|tuple| within_sensor_range(*tuple, position))
        {
            num_points_in_range += 1;
        }
    }
    num_points_in_range
}

fn part_2(input: &str, max_coord: i64) -> i64 {
    let sensors_and_beacons = parse_sensor_and_beacon_list(input);
    let radii: HashMap<(i64, i64), i64> = sensors_and_beacons
        .iter()
        .map(|tuple| ((tuple.0, tuple.1), get_radius(*tuple)))
        .collect();
    let sensors: Vec<(i64, i64)> = sensors_and_beacons
        .iter()
        .map(|tuple| (tuple.0, tuple.1))
        .collect();

    let mut line_type_a_coefficients = HashSet::new();
    let mut line_type_b_coefficients = HashSet::new();
    for ((x, y), r) in radii.iter() {
        line_type_a_coefficients.insert(y - x + r + 1);
        line_type_a_coefficients.insert(y - x - r - 1);
        line_type_b_coefficients.insert(x + y + r + 1);
        line_type_b_coefficients.insert(x + y - r - 1);
    }

    let tuning_constant = 4000000;

    for a in &line_type_a_coefficients {
        for b in &line_type_b_coefficients {
            let p = ((b - a) / 2, (a + b) / 2);
            if (0 < p.0 && p.0 < max_coord)
                && (0 < p.1 && p.1 < max_coord)
                && sensors.iter().all(|sensor| {
                    get_radius((p.0, p.1, sensor.0, sensor.1)) > *radii.get(sensor).unwrap()
                })
            {
                return tuning_constant * p.0 + p.1;
            }
        }
    }
    unreachable!();
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(26, part_1(input, 10));
    assert_eq!(56000011, part_2(input, 20));
}
