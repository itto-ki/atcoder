fn read_f64quartet() -> (f64, f64, f64, f64) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1], v[2], v[3])
}

fn main() {
    let (a, b, c, d) = read_f64quartet();
    if b / a > d / c {
        println!("TAKAHASHI");
    } else if b / a < d / c {
        println!("AOKI");
    } else {
        println!("DRAW");
    }
}
