use std::collections::HashSet;

fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_i32();
    let mut set = HashSet::new();
    let mut cnt = 0;
    for _ in 0..n {
        let a = read_i32();
        if set.contains(&a) {
            cnt += 1;
        } else {
            set.insert(a);
        }
    }
    println!("{}", cnt);
}
