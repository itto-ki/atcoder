fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let a = read_i32();
    let b = read_i32();
    if a % b == 0 {
        println!("0");
    } else {
        println!("{}", b - a % b);
    }
}
