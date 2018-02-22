fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let a = read_i32();
    let b = read_i32();
    if (a - b).abs() > 5 {
        println!("{}", 10 - (a - b).abs());
    } else {
        println!("{}", (a - b).abs());
    }
}
