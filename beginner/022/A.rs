fn read_i32triple() -> (i32, i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1], v[2])
}

fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let (n, s, t) = read_i32triple();
    let mut w = read_i32();
    let mut cnt = 0;
    for i in 0..n {
        if i != 0 {    // 初日以外の場合
            let a = read_i32();
            w += a;
        }
        if s <= w && w <= t {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
