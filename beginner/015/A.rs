fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let a = read_string();
    let b = read_string();
    if a.len() > b.len() {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}
