pub type Matrix = Vec<Vec<i32>>;

type IndexPair = (usize, usize);

type SaddlePoints = Vec<IndexPair>;

fn check_is_row_minimum(row: &Vec<i32>, value: &i32) -> bool {
    let row_option = row.iter().min();

    match row_option {
        None => false,
        Some(minimum) => minimum == value,
    }
}

fn check_is_column_maximum(input: &Matrix, column_index: usize, value: &i32) -> bool {
    let column_option = input[column_index].iter().max();

    match column_option {
        None => false,
        Some(maximum) => maximum == value,
    }
}

pub fn saddle_points(input: &Matrix) -> SaddlePoints {
    let mut result: SaddlePoints = Vec::new();

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let is_row_minimum = check_is_row_minimum(row, value);
            let is_column_maximum = check_is_column_maximum(input, column_index, value);

            if is_row_minimum && is_column_maximum {
                result.push((row_index, column_index));
            }
        }
    }

    result
}
