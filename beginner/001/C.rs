#[allow(unused_must_use)]
fn read_int_pair() -> (i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let inputs: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    (inputs[0], inputs[1])
}

fn main() {
    let (deg, dis) = read_int_pair();
    let wind_speed_average = (dis as f32) / 60.0_f32;
    println!("{}", wind_speed_average);
}
