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
    let (m, d) = read_i32pair();
    if m % d == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
