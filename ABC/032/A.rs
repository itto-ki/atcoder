fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}


fn main() {
    let a = read_i32();
    let b = read_i32();
    let mut n = read_i32();

    while n % a != 0 || n % b != 0 {
        n += 1;
    }
    println!("{}", n);
}