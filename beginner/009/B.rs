fn read_int() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_int();
    let mut price_1st = 0;
    let mut price_2nd = 0;
    for _ in 0..n {
        let price = read_int();
        if price_1st < price {
            price_2nd = price_1st;
            price_1st = price;
        } else if price < price_1st && price_2nd < price {
            price_2nd = price;
        }
    }
    println!("{}", price_2nd);
}
