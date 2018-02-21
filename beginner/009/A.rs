fn read_int() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_int();
    println!("{}", (n + 1) / 2);
}
