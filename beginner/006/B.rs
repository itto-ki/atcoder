#[allow(unused_must_use)]
fn read_usize() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_usize();
    let mut fib: Vec<i32> = vec![0; n];
    for i in 0..n {
        match i {
            0 | 1 => fib[i] = 0,
            2 => fib[i] = 1,
            _ => fib[i] = (fib[i-1] + fib[i-2] + fib[i-3]) % 10007,
        }
    }
    println!("{}", fib.last().unwrap());
}
