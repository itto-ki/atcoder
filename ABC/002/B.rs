#[allow(unused_must_use)]
fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_owned()
}

fn main() {
    let input = read_string();
    for chr in input.chars() {
        match chr {
            'a' | 'i' | 'u' | 'e' | 'o' => {},
            _ => print!("{}", chr)
        }
    }
    println!("");
}
