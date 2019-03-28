fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_i32();
    let mut cs = Vec::with_capacity(n as usize);
    for _ in 0..n {
        cs.push(read_i32());
    }
}
