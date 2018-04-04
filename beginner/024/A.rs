fn read_i32quartet() -> (i32, i32, i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1], v[2], v[3])
}

fn read_i32pair() -> (i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn main() {
    let (a, b, c, k) = read_i32quartet();
    let (s, t) = read_i32pair();
    if s + t >= k {
        println!("{}", (a - c) * s + (b - c) * t);
    } else {
        println!("{}", a * s + b * t);
    }
}
