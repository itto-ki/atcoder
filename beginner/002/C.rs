#[allow(unused_must_use)]
fn read_vecint() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let inputs = read_vecint();
    let a = inputs[2] - inputs[0];
    let b = inputs[3] - inputs[1];
    let c = inputs[4] - inputs[0];
    let d = inputs[5] - inputs[1];
    println!("{}", ((a * d - b * c).abs() as f64) * 0.5);
}
