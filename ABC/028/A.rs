fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_i32();
    if n <= 59 {
        println!("Bad");
    } else if n <= 89 {
        println!("Good");
    } else if n <= 99 {
        println!("Great");
    } else {
        println!("Perfect");
    }
}
