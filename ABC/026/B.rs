use std::f64::consts::PI;

fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_i32() as usize;
    let mut v: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(read_i32());
    }
    v.sort();
    let mut r_pow: f64 = 0.0;
    for (i, item) in v.iter().enumerate() {
        if i % 2 == 0 {
            r_pow -= (item * item) as f64;
        } else {
            r_pow += (item * item) as f64;
        }
    }
    if r_pow > 0.0 {
        println!("{}", r_pow * PI);
    } else {
        println!("{}", - (r_pow * PI));
    }
}
