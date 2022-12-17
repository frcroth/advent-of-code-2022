fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn is_list(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }
    s.starts_with('[') && s.ends_with(']')
}

// FIXME
fn get_next_element_from_list(list: String) -> (Option<String>, Option<String>) {
    if is_list(&list) {
        let mut depth = 0;
        for (i, c) in list.chars().enumerate() {
            if c == '[' {
                depth += 1;
            }
            if c == ']' {
                depth -= 1;
            }
            if c == ',' && depth == 1 {
                let (first_part, rest) = list.split_at(i);
                return (
                    first_part.split_once('[').map(|(_, b)| b.to_string()),
                    Some(format!("[{}", rest.strip_prefix(',').unwrap())),
                );
            }
        }
        if &list == "[]" {
            return (None, None);
        }
        return (
            Some(
                list.strip_prefix('[')
                    .unwrap()
                    .strip_suffix(']')
                    .unwrap()
                    .to_string(),
            ),
            None,
        );
    }

    (None, None)
}

fn in_right_order(a: String, b: String) -> Option<bool> {
    let num_a = a.parse::<u32>();
    let num_b = b.parse::<u32>();
    if let (Ok(n_a), Ok(n_b)) = (num_a.clone(), num_b.clone()) {
        if n_a < n_b {
            return Some(true);
        }
        if n_a > n_b {
            return Some(false);
        }
        return None;
    }
    if is_list(&a) && is_list(&b) {
        let mut list_a = a;
        let mut list_b = b;
        loop {
            let (e_a, l_a) = get_next_element_from_list(list_a);
            let (e_b, l_b) = get_next_element_from_list(list_b);
            if e_a.is_none() && e_b.is_some() {
                return Some(true);
            }
            if e_a.is_some() && e_b.is_none() {
                return Some(false);
            }
            if e_a.is_none() && e_b.is_none() {
                return None;
            }
            list_a = l_a.clone().unwrap_or_else(|| String::from(""));
            list_b = l_b.clone().unwrap_or_else(|| String::from(""));
            if let Some(r) = in_right_order(e_a.unwrap(), e_b.unwrap()) {
                return Some(r);
            }
        }
    }

    if let Ok(n_a) = num_a {
        return in_right_order(format!("[{}]", n_a), b);
    }
    if let Ok(n_b) = num_b {
        return in_right_order(a, format!("[{}]", n_b));
    }
    unreachable!("Got to the end!")
}

fn part_1(input: &str) -> usize {
    let problems = input.split("\n\n").map(|lists| {
        let (a, b) = lists.split_once('\n').unwrap();
        (a.to_string(), b.to_string())
    });
    problems
        .enumerate()
        .filter(|(_, (a, b))| in_right_order(a.clone(), b.clone()).unwrap())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_2(input: &str) -> usize {
    let packets = input
        .split("\n\n")
        .map(|lists| {
            lists
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();
    let position_1 = 1 + packets
        .iter()
        .filter(|packet| in_right_order((**packet).clone(), String::from("[[2]]")).unwrap())
        .count();
    let position_2 = 2 + packets
        .iter()
        .filter(|packet| in_right_order((**packet).clone(), String::from("[[6]]")).unwrap())
        .count();
    position_1 * position_2
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(13, part_1(input));
    assert_eq!(140, part_2(input));
}
