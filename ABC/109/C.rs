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

fn read_vec() -> Vec<usize> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn gcd(x: usize, y: usize) -> usize {
    if y % x == 0 {
        return x;
    }
    gcd(y, y % x)
}

fn main() {
    let (n, x) = read_usizepair();
    let mut points = read_vec();
    let mut deltas = Vec::with_capacity(n);
    points.push(x);
    points.sort();
    for i in 0..n {
        deltas.push(points[i+1] - points[i]);
    }
    deltas.sort();
    if deltas.len() != 1 {
        let mut gcd_value = gcd(deltas[0], deltas[1]);
        for delta in &deltas[2..] {
            gcd_value = gcd(gcd_value, *delta);
        }
        println!("{}", gcd_value);
    } else {
        println!("{}", deltas[0]);
    }
}