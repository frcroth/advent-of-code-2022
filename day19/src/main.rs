use regex::Regex;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn parse_input(input: &str) -> Vec<(isize, isize, isize, isize, isize, isize, isize)> {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian").unwrap();
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
                caps.get(4)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
                caps.get(5)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
                caps.get(6)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
                caps.get(7)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
            )
        })
        .collect()
}

fn find_highest_geode_count(
    ore_robot_cost: isize,
    clay_robot_cost: isize,
    obsidian_robot_ore_cost: isize,
    obsidian_robot_clay_cost: isize,
    geode_robot_ore_cost: isize,
    geode_robot_obsidian_cost: isize,
    time: isize,
) -> isize {
    let mut best = 0;
    let initial_state = (0, 0, 0, 0, 1, 0, 0, 0, time);
    let mut visit_queue = VecDeque::new();
    let mut visited = HashSet::new();
    visit_queue.push_back(initial_state);

    while let Some(state) = visit_queue.pop_front() {
        let (
            ore,
            clay,
            obsidian,
            geodes,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
            time,
        ) = state;
        best = best.max(geodes);

        if time == 0 || time * geodes + isize::max((time - 2) * (time - 1) / 2, 0) < best {
            continue;
        }

        let max_ore_cost = [
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_ore_cost,
            geode_robot_ore_cost,
        ]
        .iter()
        .cloned()
        .max()
        .unwrap();
        // Since only one ore robot can be built per minute, we can not use more than the maximal ore cost of any robot in ore per minute.

        let useful_ore_robots = if ore_robots >= max_ore_cost {
            max_ore_cost
        } else {
            ore_robots
        };

        let useful_clay_robots = if clay_robots >= obsidian_robot_clay_cost {
            obsidian_robot_clay_cost
        } else {
            clay_robots
        };
        let useful_obsidian_robots = if obsidian_robots >= geode_robot_obsidian_cost {
            geode_robot_obsidian_cost
        } else {
            obsidian_robots
        };
        // It always makes sense to build more geode robots

        let useful_ore = if ore >= time * max_ore_cost - useful_ore_robots * (time - 1) {
            time * max_ore_cost - useful_ore_robots * (time - 1)
        } else {
            ore
        };
        let useful_clay =
            if clay >= time * obsidian_robot_clay_cost - useful_clay_robots * (time - 1) {
                time * obsidian_robot_clay_cost - useful_clay_robots * (time - 1)
            } else {
                clay
            };
        let useful_obsidian =
            if obsidian >= time * geode_robot_obsidian_cost - useful_obsidian_robots * (time - 1) {
                time * geode_robot_obsidian_cost - useful_obsidian_robots * (time - 1)
            } else {
                obsidian
            };

        let state = (
            useful_ore,
            useful_clay,
            useful_obsidian,
            geodes,
            useful_ore_robots,
            useful_clay_robots,
            useful_obsidian_robots,
            geode_robots,
            time,
        );

        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        visit_queue.push_back((
            useful_ore + useful_ore_robots,
            useful_clay + useful_clay_robots,
            useful_obsidian + useful_obsidian_robots,
            geodes + geode_robots,
            useful_ore_robots,
            useful_clay_robots,
            useful_obsidian_robots,
            geode_robots,
            time - 1,
        ));
        if useful_ore >= ore_robot_cost {
            visit_queue.push_back((
                useful_ore - ore_robot_cost + useful_ore_robots,
                useful_clay + useful_clay_robots,
                useful_obsidian + useful_obsidian_robots,
                geodes + geode_robots,
                useful_ore_robots + 1,
                useful_clay_robots,
                useful_obsidian_robots,
                geode_robots,
                time - 1,
            ));
        }
        if useful_ore >= clay_robot_cost {
            visit_queue.push_back((
                useful_ore - clay_robot_cost + useful_ore_robots,
                useful_clay + useful_clay_robots,
                useful_obsidian + useful_obsidian_robots,
                geodes + geode_robots,
                useful_ore_robots,
                useful_clay_robots + 1,
                useful_obsidian_robots,
                geode_robots,
                time - 1,
            ));
        }
        if useful_ore >= obsidian_robot_ore_cost && useful_clay >= obsidian_robot_clay_cost {
            visit_queue.push_back((
                useful_ore - obsidian_robot_ore_cost + useful_ore_robots,
                useful_clay - obsidian_robot_clay_cost + useful_clay_robots,
                useful_obsidian + useful_obsidian_robots,
                geodes + geode_robots,
                useful_ore_robots,
                useful_clay_robots,
                useful_obsidian_robots + 1,
                geode_robots,
                time - 1,
            ));
        }
        if useful_ore >= geode_robot_ore_cost && useful_obsidian >= geode_robot_obsidian_cost {
            visit_queue.push_back((
                useful_ore - geode_robot_ore_cost + useful_ore_robots,
                useful_clay + useful_clay_robots,
                useful_obsidian - geode_robot_obsidian_cost + useful_obsidian_robots,
                geodes + geode_robots,
                useful_ore_robots,
                useful_clay_robots,
                useful_obsidian_robots,
                geode_robots + 1,
                time - 1,
            ));
        }
    }
    best
}

fn part_1(input: &str) -> isize {
    let blueprints = parse_input(input);
    blueprints.into_iter().fold(0, |acc, blueprint| {
        acc + find_highest_geode_count(
            blueprint.1,
            blueprint.2,
            blueprint.3,
            blueprint.4,
            blueprint.5,
            blueprint.6,
            24,
        ) * blueprint.0
    })
}

fn part_2(input: &str) -> isize {
    let blueprints = parse_input(input);
    let mut total = 1;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let geode_count = find_highest_geode_count(
            blueprint.1,
            blueprint.2,
            blueprint.3,
            blueprint.4,
            blueprint.5,
            blueprint.6,
            32,
        );
        if i > 2 {
            break; // Break to allow example with only 2 blueprints
        }
        total *= geode_count;
    }
    total
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(33, part_1(input));
    assert_eq!(3472, part_2(input));
}
