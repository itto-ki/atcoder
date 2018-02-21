fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let mut s = read_string();
    s.push_str("pp");
    println!("{}", s);
}
