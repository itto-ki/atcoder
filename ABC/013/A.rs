fn read_char() -> char {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let x = read_char();
    println!("{}", (x as u8) - ('A' as u8) + 1);
}
