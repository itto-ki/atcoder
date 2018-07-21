fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let q = read_i32();
    if q == 1 {
        println!("ABC");
    } else {
        println!("chokudai");
    }
}
