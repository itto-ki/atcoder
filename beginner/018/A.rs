fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn main() {
    let a = read_i32();
    let b = read_i32();
    let c = read_i32();
    let mut ranking = vec![a, b, c];
    ranking.sort();
    ranking.reverse();
    let nth = |x: &i32, ranking: &Vec<i32>| -> usize {
        for (i, y) in ranking.iter().enumerate() {
            if x == y {
                return i + 1;
            }
        }
        0
    };
    println!("{}", nth(&a, &ranking));
    println!("{}", nth(&b, &ranking));
    println!("{}", nth(&c, &ranking));
}
