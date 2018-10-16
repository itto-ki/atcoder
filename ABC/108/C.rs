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
    let (n, k) = read_usizepair();

    if k % 2 != 0 {
        println!("{}", ((n / k) as i64).pow(3))
    } else {
        println!("{}", ((n / k) as i64).pow(3) + (((n + k / 2) / k) as i64).pow(3));
    }
}
