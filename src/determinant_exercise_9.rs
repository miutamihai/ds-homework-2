pub type Matrix = Vec<Vec<i32>>;

type IndexPair = (usize, usize);

fn get_dimensions(input: &Matrix) -> IndexPair {
    let column_count = input.iter().map(|column| column.len()).max();

    match column_count {
        None => (0, 0), // will never happen, the compiler demands this case is handled too
        Some(columns) => (input.len(), columns),
    }
}

fn initialize_sub_matrix(input: &Matrix) -> Matrix {
    let (rows, columns): IndexPair = get_dimensions(input);

    // Creates a vector of vectors of size rows * columns, all values being 0
    vec![vec![0; columns]; rows]
}

pub fn determinant(input: &Matrix) -> i32 {
    let matrix_order: usize = input.len();
    let mut sub_matrix: Matrix = initialize_sub_matrix(&input);
    let mut result: i32 = 0;

    if matrix_order == 2 {
        return input[0][0] * input[1][1] - input[1][0] * input[0][1];
    }

    for x in 0..matrix_order {
        let mut sub_i: usize = 0;

        for i in 0..matrix_order {
            let mut sub_j: usize = 0;

            for j in 0..matrix_order {
                if j != x {
                    sub_matrix[sub_i][sub_j] = input[i][j];
                    sub_j = sub_j + 1;
                }
            }

            sub_i = sub_i + 1;
        }

        result = result + ((-1 as i32).pow(x as u32) * input[0][x] * determinant(&sub_matrix));
    }

    result
}
