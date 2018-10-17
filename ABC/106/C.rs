fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn read_usize() -> usize {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse::<usize>().unwrap()
}

fn main() {
    let s = read_string();
    let mut chars = s.chars();
    let k = read_usize();
    let mut i: usize = 0;

    while let Some(c) = chars.next() {
        if i >= k {
            break;
        }
        if c != '1' {
            println!("{}", c);
            return;
        }
        i += 1;
    }
    println!("1");
}