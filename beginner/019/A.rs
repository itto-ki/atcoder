fn read_i32vec() -> Vec<i32> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let mut ages = read_i32vec();
    ages.sort();
    println!("{}", ages[1]);
}
