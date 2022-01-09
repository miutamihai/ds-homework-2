type Pair = (i32, i32);

fn display_pair(pair: Pair) -> () {
    println!("{} / {}", pair.0, pair.1);
}

pub fn farey(n: i32) -> () {
    let mut result: Vec<Pair> = vec![(0, 1)];
    let (mut a, mut b, mut c, mut d) = (0, 1, 1, n);

    while c <= n {
        let k = (n + b) / d;
        let next_c = k * c - a;
        let next_d = k * d - b;

        a = c;
        b = d;
        c = next_c;
        d = next_d;

        result.push((a, b));
    }

    for i in 0..result.len() {
        display_pair(result[i]);
    }
}
