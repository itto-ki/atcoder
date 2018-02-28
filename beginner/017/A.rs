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
    let (s1, e1) = read_i32pair();
    let (s2, e2) = read_i32pair();
    let (s3, e3) = read_i32pair();
    let score = |s: i32, e: i32 | -> i32 { (((s * e) as f32) * 0.1) as i32 };
    println!("{}", score(s1, e1) + score(s2, e2) + score(s3, e3));
}
