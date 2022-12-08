use array2d::Array2D;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    let tree_matrix = parse_input_to_matrix(input);
    let dim = tree_matrix.column_len();
    let mut visible_matrix = Array2D::filled_with(false, dim, dim);

    for (y, row) in tree_matrix.rows_iter().enumerate() {
        let mut current_max = -1;
        for (x, tree) in row.enumerate() {
            if *tree > current_max {
                current_max = *tree;
                visible_matrix.set(y, x, true).unwrap();
            }
        }
        // right to left

        current_max = -1;
        for x in (0..dim).rev() {
            let tree = tree_matrix.get(y, x).unwrap();
            if *tree > current_max {
                current_max = *tree;
                visible_matrix.set(y, x, true).unwrap();
            }
        }
    }

    for (x, col) in tree_matrix.columns_iter().enumerate() {
        let mut current_max = -1;
        for (y, tree) in col.enumerate() {
            if *tree > current_max {
                current_max = *tree;
                visible_matrix.set(y, x, true).unwrap();
            }
        }
        // down to up
        current_max = -1;
        for y in (0..dim).rev() {
            let tree = tree_matrix.get(y, x).unwrap();
            if *tree > current_max {
                current_max = *tree;
                visible_matrix.set(y, x, true).unwrap();
            }
        }
    }
    visible_matrix.as_row_major().iter().filter(|b| **b).count() as u32
}

fn parse_input_to_matrix(input: &str) -> Array2D<i32> {
    Array2D::from_rows(
        &input
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>(),
    )
}

fn calculate_scenic_score(matrix: &Array2D<i32>, row: usize, col: usize) -> i32 {
    let dim = matrix.column_len();
    if row == 0 || col == 0 || row == dim - 1 || col == dim - 1 {
        return 0;
    }

    let score_accumulator = |(count, stop), element| -> (i32, bool) {
        match stop {
            true => (count, stop),
            false => (count + 1, element >= matrix.get(row, col).unwrap()),
        }
    };

    let row_vec = matrix.row_iter(row).cloned().collect::<Vec<i32>>();
    let col_vec = matrix.column_iter(col).cloned().collect::<Vec<i32>>();
    let mut row_vec_rev = row_vec[0..col].to_vec();
    row_vec_rev.reverse();
    let mut col_vec_rev = col_vec[0..row].to_vec();
    col_vec_rev.reverse();

    vec![
        row_vec_rev.iter().fold((0, false), score_accumulator).0,
        row_vec[col + 1..dim]
            .iter()
            .fold((0, false), score_accumulator)
            .0,
        col_vec_rev.iter().fold((0, false), score_accumulator).0,
        col_vec[(row + 1)..dim]
            .iter()
            .fold((0, false), score_accumulator)
            .0,
    ]
    .iter()
    .product()
}

fn part_2(input: &str) -> i32 {
    let tree_matrix = parse_input_to_matrix(input);
    let dim = tree_matrix.column_len();
    let mut best_view = i32::MIN;
    for y in 0..dim - 1 {
        for x in 0..dim - 1 {
            best_view = i32::max(best_view, calculate_scenic_score(&tree_matrix, y, x))
        }
    }
    best_view
}

#[test]
fn test_example() {
    let input = include_str!("../example.txt");

    assert_eq!(21, part_1(input));
    assert_eq!(8, part_2(input));
}

/*
fn print_array(array: &Array2D<bool>) {
    for row in array.rows_iter() {
        let string = row
            .map(|b| match b {
                true => "1",
                false => "0",
            })
            .fold(String::from(""), |acc, elem| format!("{}{}", acc, elem));
        println!("{}", string);
    }
    println!("\n");
}
*/
