fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(parse_inventory_list(input)));
    println!("{}", part_2(parse_inventory_list(input)));
}

fn parse_inventory_list(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split('\n')
                .map(|x| x.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn part_1(inventory_list: Vec<Vec<u32>>) -> u32 {
    inventory_list
        .iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .unwrap()
}

fn part_2(inventory_list: Vec<Vec<u32>>) -> u32 {
    let mut sums = inventory_list
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect::<Vec<u32>>();
    sums.sort_unstable();
    sums.reverse();
    sums[0] + sums[1] + sums[2]
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(24000, part_1(parse_inventory_list(input)));
    assert_eq!(45000, part_2(parse_inventory_list(input)));
}
