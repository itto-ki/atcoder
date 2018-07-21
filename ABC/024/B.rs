fn read_usizepair() -> (usize, usize) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<usize> = input.trim()
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
    let (n, t) = read_usizepair();
    let mut rslt = 0;
    let mut a_vec: Vec<usize> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        a_vec.push(read_usize());
    }
    for i in 0..(n - 1) {
        if a_vec[i] <= a_vec[i + 1] && a_vec[i + 1] <= a_vec[i] + t {
            rslt += a_vec[i + 1] - a_vec[i];
        } else {
            rslt += t;
        }
    }
    println!("{}", rslt + t); // 最後に来た人の後t秒間は開き続ける
}
