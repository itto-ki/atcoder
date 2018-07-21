#[allow(unused_must_use)]
fn read_int() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let first_line = read_int();
    let (n, k) = (first_line[0], first_line[1]);
    let mut rates = read_int();
    rates.sort();
    let mut c = 0.0;
    for i in (n - k)..n {
        c = (c + rates[i as usize] as f32) / 2.0;
    }
    println!("{}", c);
}
