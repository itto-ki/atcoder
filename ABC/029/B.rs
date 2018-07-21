const N: usize = 12;

fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let mut cnt = 0;
    for _ in 0..N {
        let s = read_string();
        if s.contains("r") {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
