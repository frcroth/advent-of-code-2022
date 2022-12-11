fn main() {
    println!("{}", part_1(get_input()));
    println!("{}", part_2(get_input()));
}

#[derive(Copy, Clone)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

impl Operation {
    fn value(&self, old: u64) -> u64 {
        match self {
            Operation::Add(v) => v + old,
            Operation::Mult(v) => v * old,
            Operation::Square => old * old,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation_mode: Operation,
    division_test: u64,
    true_monkey_index: usize,
    false_monkey_index: usize,
}

#[allow(dead_code)]
fn get_example() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            operation_mode: Operation::Mult(19),
            division_test: 23,
            true_monkey_index: 2,
            false_monkey_index: 3,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation_mode: Operation::Add(6),
            division_test: 19,
            true_monkey_index: 2,
            false_monkey_index: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation_mode: Operation::Square,
            division_test: 13,
            true_monkey_index: 1,
            false_monkey_index: 3,
        },
        Monkey {
            items: vec![74],
            operation_mode: Operation::Add(3),
            division_test: 17,
            true_monkey_index: 0,
            false_monkey_index: 1,
        },
    ]
}

// No parsing today
fn get_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![99, 67, 92, 61, 83, 64, 98],
            operation_mode: Operation::Mult(17),
            division_test: 3,
            true_monkey_index: 4,
            false_monkey_index: 2,
        },
        Monkey {
            items: vec![78, 74, 88, 89, 50],
            operation_mode: Operation::Mult(11),
            division_test: 5,
            true_monkey_index: 3,
            false_monkey_index: 5,
        },
        Monkey {
            items: vec![98, 91],
            operation_mode: Operation::Add(4),
            division_test: 2,
            true_monkey_index: 6,
            false_monkey_index: 4,
        },
        Monkey {
            items: vec![59, 72, 94, 91, 79, 88, 94, 51],
            operation_mode: Operation::Square,
            division_test: 13,
            true_monkey_index: 0,
            false_monkey_index: 5,
        },
        Monkey {
            items: vec![95, 72, 78],
            operation_mode: Operation::Add(7),
            division_test: 11,
            true_monkey_index: 7,
            false_monkey_index: 6,
        },
        Monkey {
            items: vec![76],
            operation_mode: Operation::Add(8),
            division_test: 17,
            true_monkey_index: 0,
            false_monkey_index: 2,
        },
        Monkey {
            items: vec![69, 60, 53, 89, 71, 88],
            operation_mode: Operation::Add(5),
            division_test: 19,
            true_monkey_index: 7,
            false_monkey_index: 1,
        },
        Monkey {
            items: vec![72, 54, 63, 80],
            operation_mode: Operation::Add(3),
            division_test: 7,
            true_monkey_index: 1,
            false_monkey_index: 3,
        },
    ]
}

fn part_1(setup: Vec<Monkey>) -> u64 {
    perform_rounds(setup, 20, true)
}

fn part_2(setup: Vec<Monkey>) -> u64 {
    perform_rounds(setup, 10000, false)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn perform_rounds(setup: Vec<Monkey>, round_count: usize, part_1: bool) -> u64 {
    let mut monkeys = setup;
    let mut inspection_counter = vec![];
    inspection_counter.resize(monkeys.len(), 0);
    let lcm = monkeys.iter().map(|m| m.division_test).reduce(lcm).unwrap();

    for _round in 0..round_count {
        for i in 0..monkeys.len() {
            let current_monkey = monkeys[i].clone();
            let items = current_monkey.items;
            monkeys[i].items = vec![];
            for item in items {
                inspection_counter[i] += 1;
                let new_worry_level = if part_1 {
                    current_monkey.operation_mode.value(item) / 3
                } else {
                    current_monkey.operation_mode.value(item) % lcm
                };
                if new_worry_level % current_monkey.division_test == 0 {
                    monkeys[current_monkey.true_monkey_index]
                        .items
                        .push(new_worry_level);
                } else {
                    monkeys[current_monkey.false_monkey_index]
                        .items
                        .push(new_worry_level);
                }
            }
        }
        /*
        if !part_1 && (_round+1 == 1 || ((_round+1) % 1000 == 0)) {
            println!("After round {} the monkeys are holding items with these worry levels:", _round + 1);
            for i in 0..monkeys.len() {
                println!("Monkey {}: {}", i, monkeys[i].items.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", "));
            }
            for c in 0..inspection_counter.len() {
                    println!("Monkey {} inspected items {} times.", c, inspection_counter[c]);
                }
        }*/
    }

    inspection_counter.sort();
    inspection_counter.reverse();
    inspection_counter[0] * inspection_counter[1]
}

#[test]
fn test_example() {
    assert_eq!(10605, part_1(get_example()));
    assert_eq!(2713310158, part_2(get_example()));
}
