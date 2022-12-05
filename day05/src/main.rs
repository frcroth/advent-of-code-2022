use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input).as_str());
    println!("{}", part_2(input).as_str());
}

struct Command {
    amount: u32,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let split = input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let initial_stack_layout = split[0].to_string();
    let stack_count: usize = (initial_stack_layout
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[0]
        .len()
        / 3)
        + 1;
    let mut stacks = (0..stack_count).map(|_| vec![]).collect::<Vec<Vec<char>>>();
    let initial_stack_layout_len = initial_stack_layout.split('\n').count();
    initial_stack_layout
        .split('\n')
        .enumerate()
        .for_each(|(i, line)| {
            if i < initial_stack_layout_len - 1 {
                line.to_string()
                    .chars()
                    .collect::<Vec<char>>()
                    .as_slice()
                    .chunks(4)
                    .enumerate()
                    .for_each(|(i, stack_content)| {
                        if stack_content.iter().collect::<String>() != *"    " {
                            stacks[i]
                                .push((*stack_content).iter().copied().collect::<Vec<char>>()[1])
                        }
                    })
            }
        });

    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let commands = split[1]
        .to_string()
        .split('\n')
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Command {
                amount: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                from: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Command>>();
    (
        stacks
            .into_iter()
            .map(|s| s.into_iter().rev().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
        commands,
    )
}

fn print_top_elements(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.last().unwrap())
        .collect::<String>()
}

fn part_1(input: &str) -> String {
    let (mut stacks, commands) = parse_input(input);
    for command in commands {
        let Command { amount, from, to } = command;

        for _ in 0..amount {
            let e = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(e);
        }
    }
    print_top_elements(&stacks)
}

fn part_2(input: &str) -> String {
    let (mut stacks, commands) = parse_input(input);
    for command in commands {
        let Command { amount, from, to } = command;
        let mut elements = vec![];
        for _ in 0..amount {
            elements.push(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..amount {
            stacks[to - 1].push(elements.pop().unwrap())
        }
    }
    print_top_elements(&stacks)
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!("CMZ", part_1(input).as_str());
    assert_eq!("MCD", part_2(input).as_str());
}
