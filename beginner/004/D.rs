#[allow(unused_must_use)]
fn read_int_triple() -> (i32, i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1], v[2])
}

fn count_move_marble(number_of_marble: i32) -> i32 {
    let factorial = |x: i32| -> i32 { (0..x+1).fold(0, |acc, x| acc + x) };
    if number_of_marble % 2 == 0 {
        let right = number_of_marble / 2;
        let left = number_of_marble / 2 - 1;
        factorial(right) + factorial(left)
    } else {
        let both = number_of_marble / 2;
        factorial(both) * 2
    }
}

fn main() {
    let (r, g, b) = read_int_triple();
    println!("{} {} {}", r, g, b);
    let total = count_move_marble(r) + count_move_marble(g) + count_move_marble(b);
    println!("r:{}, g:{}, b:{}", count_move_marble(r), count_move_marble(g), count_move_marble(b));
    println!("{}", total);
}
