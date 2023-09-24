use std::collections::{hash_map::Entry, HashMap};

// Based on solution by u/Gix

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

enum JetDirection {
    Left,
    Right,
}

fn parse_jets(input: &str) -> Vec<JetDirection> {
    input
        .chars()
        .map(|c| match c {
            '>' => JetDirection::Right,
            '<' => JetDirection::Left,
            _ => panic!(),
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let jets_directions = parse_jets(input);
    let mut jet_index = 0;
    let num_rocks = 2022;
    let mut cave = Vec::with_capacity(num_rocks * 4);
    for shape in RockShape::all_shapes().into_iter().cycle().take(num_rocks) {
        jet_index = drop_rock(&mut cave, &jets_directions, jet_index, shape);
    }
    cave.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RockShape(u32);

impl RockShape {
    const fn all_shapes() -> [Self; 5] {
        [
            // Each line is 1 byte,
            // First bit always 0 (left wall)
            // Bits 2,3 are 0 because rocks start with a left offset of 2
            // Lines are shifted by 8 to fit 4 lines into one 32-bit word
            Self(0b0011110),
            Self(0b0001000 ^ 0b0011100 << 8 ^ 0b0001000 << 16),
            Self(0b0011100 ^ 0b00000100 << 8 ^ 0b00000100 << 16),
            Self(0b00010000 ^ 0b10000 << 8 ^ 0b10000 << 16 ^ 0b10000 << 24),
            Self(0b00011000 ^ 0b00011000 << 8),
        ]
    }

    fn blow(&mut self, direction: &JetDirection, mask: u32) {
        let new_pos = match direction {
            JetDirection::Left => {
                if self.0 & 0x40404040 == 0 {
                    self.0 << 1
                } else {
                    return;
                }
            }
            JetDirection::Right => {
                if self.0 & 0x01010101 == 0 {
                    self.0 >> 1
                } else {
                    return;
                }
            }
        };

        if new_pos & mask == 0 {
            self.0 = new_pos;
        }
    }

    const fn intersects(&self, mask: u32) -> bool {
        self.0 & mask != 0
    }

    fn as_bytes(self) -> impl Iterator<Item = u8> {
        self.0.to_le_bytes().into_iter().take_while(|b| *b != 0)
    }
}

fn tower_mask(tower: &[u8], height: usize) -> u32 {
    if height >= tower.len() {
        0
    } else {
        tower[height..]
            .iter()
            .take(4)
            .rev()
            .fold(0u32, |acc, b| (acc << 8) | *b as u32)
    }
}

fn drop_rock(
    tower: &mut Vec<u8>,
    jets_directions: &Vec<JetDirection>,
    mut jet_index: usize,
    mut shape: RockShape,
) -> usize {
    let mut height = tower.len() + 3;

    loop {
        let jets_dir = &jets_directions[jet_index];
        jet_index = (jet_index + 1) % jets_directions.len();

        let current_mask = tower_mask(tower, height);

        shape.blow(jets_dir, current_mask);

        if height > tower.len() {
            height -= 1;
        } else if height == 0 || shape.intersects(tower_mask(tower, height - 1)) {
            for byte in shape.as_bytes() {
                if height < tower.len() {
                    tower[height] |= byte;
                } else {
                    tower.push(byte);
                }
                height += 1;
            }
            return jet_index;
        } else {
            height -= 1;
        }
    }
}

fn part_2(input: &str) -> usize {
    let rock_count: usize = 1_000_000_000_000;
    let mut seen_states = HashMap::with_capacity(1024);
    let jets_directions = parse_jets(input);
    let mut tower = Vec::with_capacity(1024);

    let mut cycle_height = 0;
    let mut jets_index = 0;
    let shapes = RockShape::all_shapes();
    let mut n = 0;

    while n < rock_count {
        let shape_index = n % shapes.len();
        let shape = shapes[shape_index];

        jets_index = drop_rock(&mut tower, &jets_directions, jets_index, shape);
        n += 1;

        if tower.len() < 8 {
            continue;
        }

        let skyline_bytes = &tower[tower.len() - 8..];
        let mut skyline: u64 = 0;
        for (i, &byte) in skyline_bytes.iter().enumerate() {
            skyline |= (byte as u64) << (i * 8);
        }

        let state = (skyline, shape_index, jets_index);

        match seen_states.entry(state) {
            Entry::Occupied(e) => {
                let (old_n, old_height) = e.get();
                let num_rocks_in_cycle = n - old_n;
                let num_cycles = (rock_count - n) / num_rocks_in_cycle;
                n += num_rocks_in_cycle * num_cycles;
                cycle_height += num_cycles * (tower.len() - old_height);
                seen_states = HashMap::with_capacity(1024); // Create a new HashMap
            }
            Entry::Vacant(e) => {
                e.insert((n, tower.len()));
            }
        }
    }

    tower.len() + cycle_height
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(3068, part_1(input));
    assert_eq!(1514285714288, part_2(input));
}
