use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

enum MonkeyInstruction {
    Value(isize),
    Plus(String, String),
    Minus(String, String),
    Multiply(String, String),
    Divide(String, String),
}

struct Monkey {
    name: String,
    instruction: MonkeyInstruction,
}

fn parse_monkey_instructions(input: &str) -> Vec<Monkey> {
    input
        .split('\n')
        .map(|line| {
            let parts = line.split(':').collect::<Vec<&str>>();
            let name = parts[0].to_string();
            if parts.len() == 2 {
                let instruction_parts = parts[1].trim().split(' ').collect::<Vec<&str>>();
                let instruction = match instruction_parts.len() {
                    1 => MonkeyInstruction::Value(instruction_parts[0].parse::<isize>().unwrap()),
                    3 => {
                        let left = instruction_parts[0].to_string();
                        let right = instruction_parts[2].to_string();
                        match instruction_parts[1] {
                            "+" => MonkeyInstruction::Plus(left, right),
                            "-" => MonkeyInstruction::Minus(left, right),
                            "*" => MonkeyInstruction::Multiply(left, right),
                            "/" => MonkeyInstruction::Divide(left, right),
                            _ => panic!("Unknown instruction: {}", instruction_parts[1]),
                        }
                    }
                    _ => panic!("Unknown instruction: {}", parts[1]),
                };
                Monkey { name, instruction }
            } else {
                Monkey {
                    name,
                    instruction: MonkeyInstruction::Value(0),
                }
            }
        })
        .collect()
}

fn part_1(input: &str) -> isize {
    let monkeys = parse_monkey_instructions(input);
    // Build tree of instructions and monkeys, afterwards find the root and execute it

    let monkey_name_map = monkeys
        .iter()
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<String, &Monkey>>();
    get_value_for_monkey(monkey_name_map.get("root").unwrap(), &monkey_name_map)
}

fn get_value_for_monkey(monkey: &Monkey, monkey_name_map: &HashMap<String, &Monkey>) -> isize {
    match &monkey.instruction {
        MonkeyInstruction::Value(value) => *value,
        MonkeyInstruction::Plus(left, right) => {
            let left_value =
                get_value_for_monkey(monkey_name_map.get(left).unwrap(), monkey_name_map);
            let right_value =
                get_value_for_monkey(monkey_name_map.get(right).unwrap(), monkey_name_map);
            left_value + right_value
        }
        MonkeyInstruction::Minus(left, right) => {
            let left_value =
                get_value_for_monkey(monkey_name_map.get(left).unwrap(), monkey_name_map);
            let right_value =
                get_value_for_monkey(monkey_name_map.get(right).unwrap(), monkey_name_map);
            left_value - right_value
        }
        MonkeyInstruction::Multiply(left, right) => {
            let left_value =
                get_value_for_monkey(monkey_name_map.get(left).unwrap(), monkey_name_map);
            let right_value =
                get_value_for_monkey(monkey_name_map.get(right).unwrap(), monkey_name_map);
            left_value * right_value
        }
        MonkeyInstruction::Divide(left, right) => {
            let left_value =
                get_value_for_monkey(monkey_name_map.get(left).unwrap(), monkey_name_map);
            let right_value =
                get_value_for_monkey(monkey_name_map.get(right).unwrap(), monkey_name_map);
            left_value / right_value
        }
    }
}

fn part_2(input: &str) -> isize {
    let monkeys = parse_monkey_instructions(input);
    let monkey_name_map = monkeys
        .iter()
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<String, &Monkey>>();
    let mut root_to_human = get_path_to_monkey("root", "humn", &monkey_name_map);
    root_to_human.pop();
    if let MonkeyInstruction::Plus(left_side, right_side) =
        &monkey_name_map.get("root").unwrap().instruction
    {
        let target_value = if root_to_human.last().unwrap() == left_side {
            get_value_for_monkey(monkey_name_map.get(right_side).unwrap(), &monkey_name_map)
        } else {
            get_value_for_monkey(monkey_name_map.get(left_side).unwrap(), &monkey_name_map)
        };
        solve_riddle(target_value, &mut root_to_human, &monkey_name_map)
    } else {
        panic!();
    }
}

fn solve_riddle(
    target_value: isize,
    stack: &mut Vec<String>,
    monkey_name_map: &HashMap<String, &Monkey>,
) -> isize {
    if stack.len() == 1 {
        target_value
    } else {
        let monkey = monkey_name_map.get(&stack.pop().unwrap()).unwrap();

        match &monkey.instruction {
            MonkeyInstruction::Value(_) => unreachable!(),
            MonkeyInstruction::Plus(a, b)
            | MonkeyInstruction::Minus(a, b)
            | MonkeyInstruction::Multiply(a, b)
            | MonkeyInstruction::Divide(a, b) => {
                let (is_first, other_operand) = if a == stack.last().unwrap() {
                    (
                        true,
                        get_value_for_monkey(monkey_name_map.get(b).unwrap(), monkey_name_map),
                    )
                } else {
                    (
                        false,
                        get_value_for_monkey(monkey_name_map.get(a).unwrap(), monkey_name_map),
                    )
                };
                // Reverse operations
                let new_target_num = match monkey.instruction {
                    MonkeyInstruction::Plus(_, _) => target_value - other_operand,
                    MonkeyInstruction::Minus(_, _) => {
                        if is_first {
                            target_value + other_operand
                        } else {
                            other_operand - target_value
                        }
                    }
                    MonkeyInstruction::Multiply(_, _) => target_value / other_operand,
                    MonkeyInstruction::Divide(_, _) => {
                        if is_first {
                            target_value * other_operand
                        } else {
                            other_operand / target_value
                        }
                    }
                    _ => unreachable!(),
                };
                solve_riddle(new_target_num, stack, monkey_name_map)
            }
        }
    }
}

fn get_path_to_monkey(
    start: &str,
    end: &str,
    monkey_name_map: &HashMap<String, &Monkey>,
) -> Vec<String> {
    if start == end {
        vec![start.to_string()]
    } else {
        let start_monkey = monkey_name_map.get(start).unwrap();
        match &start_monkey.instruction {
            MonkeyInstruction::Value(_) => vec![],
            MonkeyInstruction::Plus(a, b)
            | MonkeyInstruction::Minus(a, b)
            | MonkeyInstruction::Multiply(a, b)
            | MonkeyInstruction::Divide(a, b) => {
                for next_monkey in [a, b] {
                    let mut v = get_path_to_monkey(next_monkey, end, monkey_name_map);
                    if !v.is_empty() {
                        v.push(start.to_string());
                        return v;
                    }
                }
                vec![]
            }
        }
    }
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(152, part_1(input));
    assert_eq!(301, part_2(input));
}
