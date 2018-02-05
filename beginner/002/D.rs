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
    let (n, m) = read_int_pair();
    let mut relations: Vec<(i32, i32)> = Vec::with_capacity(m as usize);
    for _ in 0..m {
        relations.push(read_int_pair());
    }
    println!("{:?}", relations);
}
