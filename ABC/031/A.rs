fn read_usizepair() -> (usize, usize) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn main() {
    let (a, d) = read_usizepair();
    if a < d {
        println!("{}", (a + 1) * d);
    } else {
        println!("{}", a * (d + 1));
    }
}