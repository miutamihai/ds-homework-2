fn get_smaller_length(first: &Vec<i32>, second: &Vec<i32>) -> usize {
    if first.len() < second.len() {
        first.len()
    } else {
        second.len()
    }
}

pub fn lesser(first: Vec<i32>, second: Vec<i32>) -> i8 {
    let smaller_length: usize = get_smaller_length(&first, &second);

    for index in 0..smaller_length {
        if first[index] < second[index] {
            return -1;
        } else if first[index] > second[index] {
            return 1;
        }
    }

    if first.len() == second.len() {
        return 0;
    } else if first.len() < second.len() {
        return 1;
    } else {
        return -1;
    }
}
