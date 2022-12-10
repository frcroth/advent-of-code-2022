fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycle_values = vec![];
    for command in input.split('\n') {
        cycle_values.push(x);
        if command == "noop" {
            continue;
        }
        let val = command.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        cycle_values.push(x);
        x += val;
    }
    cycle_values[20 - 1] * 20
        + cycle_values[60 - 1] * 60
        + cycle_values[100 - 1] * 100
        + cycle_values[140 - 1] * 140
        + cycle_values[180 - 1] * 180
        + cycle_values[220 - 1] * 220
}

fn part_2(input: &str) -> String {
    let mut cycle = 0;
    let mut sprite_index = 1;
    let mut crt = vec![];
    #[allow(clippy::explicit_counter_loop)]
    for command in input.split('\n') {
        crt.push(if i32::abs(sprite_index - (cycle % 40)) < 2 {
            '#'
        } else {
            '.'
        });
        cycle += 1;
        if command == "noop" {
            continue;
        }
        let val = command.split_once(' ').unwrap().1.parse::<i32>().unwrap();

        crt.push(if i32::abs(sprite_index - (cycle % 40)) < 2 {
            '#'
        } else {
            '.'
        });
        cycle += 1;
        sprite_index += val;
    }
    crt.chunks(40)
        .map(|chunk| {
            chunk
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");
    assert_eq!(13140, part_1(input));
}
