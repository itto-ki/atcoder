fn read_int() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn read_intvec() -> Vec<i32> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let _ = read_int();
    let a_vec = read_intvec();
    let mut rslt = 0;
    for a in a_vec.iter() {
        let mut i = 0;
        while (a - i) % 3 == 2 || (a - i) % 2 == 0 {
            i += 1;
        }
        rslt += i;
    }
    println!("{}", rslt);
}
