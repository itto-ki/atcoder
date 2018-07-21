fn read_i32pair() -> (i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn read_i32vec() -> Vec<i32> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let (n, x) = read_i32pair();
    let a_vec = read_i32vec();
    let mut rslt = 0;
    for i in 0..n {
        if 2i32.pow(i as u32) & x > 0 {
            rslt += a_vec[i as usize];
        }
    }
    println!("{}", rslt);
}
