#[allow(unused_must_use)]
fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_int();
    if n == 1 {
        println!("0");
    } else {
        println!("{}", n - 1);
    }
}
