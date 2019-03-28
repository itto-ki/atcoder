fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn read_i32vec() -> Vec<i32>
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
    .trim()
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect()
    }

fn main() {
    let n = read_i32();

