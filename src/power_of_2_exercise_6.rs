pub fn power_of_2(power: i32) -> String {
    let mut result: String = "1".parse().unwrap();

    for _ in 1..power {
        result.push('0');
    }

    result
}
