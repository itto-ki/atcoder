#[allow(unused_must_use)]
fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_int();
    let mut rslt = 100;
    let min = |x: i32, y: i32| -> i32 { if x < y { x } else { y } };
    for _ in 0..n {
        rslt = min(rslt, read_int());
    }
    println!("{}", rslt);
}
