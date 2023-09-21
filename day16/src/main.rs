use indexmap::IndexMap;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

const INPUT_LENGTH: usize = 10;

fn parse_valves(input: &str) -> Vec<(String, isize, Vec<String>)> {
    let re =
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]*)")
            .unwrap();
    input
        .split('\n')
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(2)
                    .map_or(0, |m| m.as_str().parse::<isize>().unwrap()),
                caps.get(3)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.to_string().trim().to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect()
}

const MAX_VALUE: isize = isize::MAX / 2 - 1;

type Structures = (
    IndexMap<String, Vec<String>>,
    Vec<String>,
    IndexMap<String, isize>,
    IndexMap<String, isize>,
    IndexMap<String, IndexMap<String, isize>>,
    [[isize; INPUT_LENGTH]; INPUT_LENGTH],
);

fn generate_structures(input: &str) -> Structures {
    let volcano = parse_valves(input);
    let graph = create_graph(&volcano);
    let rooms = volcano
        .iter()
        .map(|v| v.0.to_string())
        .collect::<Vec<String>>();
    let rate_map: IndexMap<String, isize> = create_rate_map(&volcano);
    let bitmap: IndexMap<String, isize> = create_bitmap(&rate_map);
    let mut distances: IndexMap<String, IndexMap<String, isize>> = IndexMap::new();

    let mut distance_matrix = create_distance_matrix(&graph, &rooms);

    // Floyd-Warshall

    for (k, _) in graph.iter().enumerate() {
        for (i, _) in graph.iter().enumerate() {
            for (j, _) in graph.iter().enumerate() {
                distance_matrix[i][j] = isize::min(
                    distance_matrix[i][j],
                    distance_matrix[i][k] + distance_matrix[k][j],
                );
            }
        }
    }

    for (a, dists_a) in distance_matrix.iter().enumerate() {
        let room_a = &rooms[a];
        let mut dists_to_a_map = IndexMap::new();
        for (b, b_value) in dists_a.iter().enumerate() {
            let room_b = &rooms[b];
            dists_to_a_map.insert(room_b.clone(), *b_value);
        }
        distances.insert(room_a.clone(), dists_to_a_map);
    }

    (graph, rooms, rate_map, bitmap, distances, distance_matrix)
}

fn part_1(input: &str) -> isize {
    let (_, _, rate_map, bitmap, distances, _) = generate_structures(input);

    *visit(
        "AA".to_string(),
        30,
        0,
        0,
        &mut IndexMap::new(),
        &rate_map,
        &bitmap,
        &distances,
    )
    .values()
    .max()
    .unwrap()
}

#[allow(clippy::too_many_arguments)]
fn visit<'a>(
    room: String,
    remaining_time: isize,
    state: isize,
    current_flow: isize,
    answer: &'a mut IndexMap<isize, isize>,
    rate_map: &'a IndexMap<String, isize>,
    bitmap: &'a IndexMap<String, isize>,
    distances: &'a IndexMap<String, IndexMap<String, isize>>,
) -> &'a mut IndexMap<isize, isize> {
    answer.insert(
        state,
        isize::max(answer.get(&state).map_or(0, |a| *a), current_flow),
    );
    for u in rate_map.iter() {
        let new_remaining_time = remaining_time - distances.get(&room).unwrap()[u.0] - 1;
        let u_mask = bitmap.get(u.0).unwrap();
        if (u_mask & state > 0) || new_remaining_time <= 0 {
            continue;
        };
        visit(
            u.0.clone(),
            new_remaining_time,
            state | u_mask,
            current_flow + new_remaining_time * rate_map.get(u.0).unwrap(),
            answer,
            rate_map,
            bitmap,
            distances,
        );
    }
    answer
}

fn create_distance_matrix(
    graph: &IndexMap<String, Vec<String>>,
    rooms: &[String],
) -> [[isize; INPUT_LENGTH]; INPUT_LENGTH] {
    let mut matrix = [[MAX_VALUE; INPUT_LENGTH]; INPUT_LENGTH];
    for (room_index, room) in rooms.iter().enumerate() {
        for (next_room_index, next_room) in rooms.iter().enumerate() {
            if graph.get(room).unwrap().contains(next_room) {
                matrix[room_index][next_room_index] = 1;
            }
        }
    }
    matrix
}

fn create_bitmap(rate_map: &IndexMap<String, isize>) -> IndexMap<String, isize> {
    let mut map = IndexMap::new();
    for (index, key) in rate_map.keys().enumerate() {
        map.insert(key.clone(), 1 << index);
    }
    map
}

fn create_graph(spec: &[(String, isize, Vec<String>)]) -> IndexMap<String, Vec<String>> {
    let mut graph = IndexMap::new();
    for room in spec.iter() {
        graph.insert(room.0.to_string(), room.2.clone());
    }
    graph
}

fn create_rate_map(spec: &[(String, isize, Vec<String>)]) -> IndexMap<String, isize> {
    let mut graph = IndexMap::new();
    for room in spec.iter() {
        if room.1 != 0 {
            graph.insert(room.0.to_string(), room.1);
        }
    }
    graph
}

fn part_2(input: &str) -> isize {
    let (_, _, rate_map, bitmap, distances, _) = generate_structures(input);

    let visit_result: IndexMap<isize, isize> = visit(
        "AA".to_string(),
        26,
        0,
        0,
        &mut IndexMap::new(),
        &rate_map,
        &bitmap,
        &distances,
    )
    .to_owned();
    visit_result
        .iter()
        .flat_map(|(k1, v1)| {
            visit_result.iter().filter_map(
                move |(k2, v2)| {
                    if k1 & k2 == 0 {
                        Some(v1 + v2)
                    } else {
                        None
                    }
                },
            )
        })
        .max()
        .unwrap_or(0)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    // Change INPUT_LENGTH to 10 to run tests
    assert_eq!(1651, part_1(input));
    assert_eq!(1707, part_2(input));
}
