pub type Matrix = Vec<Vec<i32>>;

type IndexPair = (usize, usize);

fn get_dimensions(input: &Matrix) -> IndexPair {
    let column_count = input.iter().map(|column| column.len()).max();

    match column_count {
        None => (0, 0), // will never happen, the compiler demands this case is handled too
        Some(columns) => (input.len(), columns),
    }
}

fn initialize_output_matrix(input: &Matrix) -> Matrix {
    let (rows, columns): IndexPair = get_dimensions(input);

    // Creates a vector of vectors of size rows * columns, all values being 0
    vec![vec![0; columns]; rows]
}

fn get_total_element_count(input: &Matrix) -> usize {
    input.iter().map(|row| row.iter()).count()
}

pub fn rare_matrix_reverse(input: Matrix) -> Matrix {
    let mut output: Matrix = initialize_output_matrix(&input);

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            output[column_index][row_index] = *value;
        }
    }

    output
}
