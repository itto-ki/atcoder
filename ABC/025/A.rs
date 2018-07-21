fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn read_usize() -> usize {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let s = read_string();
    let n = read_usize();

    if n % s.len() == 0 {
        print!("{}", s.chars().nth(n / s.len() - 1).unwrap());
    } else {
        print!("{}", s.chars().nth(n / s.len()).unwrap());
    }
    println!("{}", s.chars().nth((n - 1) % s.len()).unwrap());
}
