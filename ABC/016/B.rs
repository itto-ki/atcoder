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

fn main() {
    let (a, b, c) = read_i32triple();
    if a + b != c && a - b != c {
        println!("!");
    } else if a + b == c && a - b == c {
        println!("?");
    } else if a + b == c {
        println!("+");
    } else {
        println!("-");
    }
}
