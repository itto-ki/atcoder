fn read_i32pair() -> (i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn main() {
    let (n, m) = read_i32pair();
    let h = if n >= 12 { n - 12 } else { n };
    let longhand_degree_per_min: f64 = 360.0 / 60.0;
    let shorthand_degree_per_hour: f64 = 360.0 / 12.0;
    let shorthand_degree_per_min: f64 = 360.0 / 12.0 / 60.0;
    let degree_between_hands =
        ((shorthand_degree_per_hour * (h as f64) + shorthand_degree_per_min * (m as f64)) - longhand_degree_per_min * (m as f64)).abs();
    if degree_between_hands > 180.0 {
        println!("{:3.04}", 360.0 - degree_between_hands);
    } else {
        println!("{:3.04}", degree_between_hands);
    }
}
