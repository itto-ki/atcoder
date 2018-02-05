#[allow(unused_must_use)]
fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_int();
    println!("{}", ((n as f32) + 1.0) / 2.0 * 10000.0);
}
