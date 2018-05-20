fn read_i32triple() -> (i32, i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1], v[2])
}

fn main() {
    let (l1, l2, l3) = read_i32triple();
    if l1 == l2 {
        println!("{}", l3);
    } else if l1 == l3 {
        println!("{}", l2);
    } else {
        println!("{}", l1);
    }
}
