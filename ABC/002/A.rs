#[allow(unused_must_use)]
fn read_int_pair() -> (i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn main() {
    let (x, y) = read_int_pair();
    if x > y {
        println!("{}", x);
    } else {
        println!("{}", y);
    }
}
