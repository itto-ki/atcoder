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

fn wind_power(dis: i32) -> i32 {
    let dis = ((dis as f32) / 60.0_f32 * 10.0_f32).round();
    let range = [
        0.0, 0.2, 0.3, 1.5, 1.6, 3.3, 3.4, 5.4, 5.5, 7.9,
        8.0, 10.7, 10.8, 13.8, 13.9, 17.1, 17.2, 20.7, 20.8, 24.4,
        24.5, 28.4, 28.5, 32.6, 32.7,
    ];
    let range: Vec<f32> = range.iter().map(|x| x * 10_f32).collect();
    for i in (0..range.len()-1).filter(|&x| x % 2 == 0) {
        if range[i] <= dis && dis <= range[i+1] {
            return (i as i32) / 2
        }
    }
    12
}

fn main() {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE",
        "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW",
    ];
    let (deg, dis) = read_int_pair();
    let direction = ((deg * 10 + 1125) / 2250 % 16) as usize;
    let w = wind_power(dis);
    if w == 0 {
        println!("C 0");
    } else {
        println!("{} {}", directions[direction], w);
    }
}
