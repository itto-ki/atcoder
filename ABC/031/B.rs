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

fn read_usize() -> usize {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let (l, h) = read_usizepair();
    let n = read_usize();
    for _ in 0..n {
        let time = read_usize();
        if l <= time && time <= h {
            println!("0");
        } else if time < l {
            println!("{}", l - time);
        } else {
            println!("-1");
        }
    }
}