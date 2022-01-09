pub type Matrix = Vec<Vec<i32>>;

fn reflexive(input: &Matrix) -> bool {
    for index in 0..input.len() {
        if input[index][index] != 1 {
            return false;
        }
    }

    true
}

fn symmetrical(input: &Matrix) -> bool {
    let fails_condition = |row_index: usize, column_index: usize| -> bool {
        input[row_index][column_index] != 1 || input[column_index][row_index] != 1
    };

    for (row_index, row) in input.iter().enumerate() {
        for (column_index, _) in row.iter().enumerate() {
            if row_index != column_index {
                if fails_condition(row_index, column_index) {
                    return false;
                }
            }
        }
    }

    true
}

fn transient(input: &Matrix) -> bool {
    let fails_condition = |i: usize, j: usize, k: usize| -> bool {
        input[i][j] == 1 && input[j][k] == 1 && input[i][k] != 1
    };

    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            for (k, _) in input.iter().enumerate() {
                if fails_condition(i, j, k) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn relationship(input: &Matrix) -> () {
    if reflexive(input) {
        println!("Reflexive relationship");
    } else {
        println!("Non reflexive relationship");
    }

    if symmetrical(input) {
        println!("Symmetrical relationship");
    } else {
        println!("Non symmetrical relationship");
    }

    if transient(input) {
        println!("Transient relationship");
    } else {
        println!("Non transient relationship");
    }
}
