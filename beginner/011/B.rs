fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let s = read_string();
    for (i, chr) in s.chars().enumerate() {
        if i == 0 {
            print!("{}", chr.to_uppercase());
        } else {
            print!("{}", chr.to_lowercase());
        }
    }
    println!("");
}
