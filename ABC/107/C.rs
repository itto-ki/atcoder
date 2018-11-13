fn read_usizepair() -> (usize, usize) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn read_vec() -> Vec<i64> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let (n, k) = read_usizepair();
    let x_list = read_vec();
    let mut cost: i64 = i64::max_value();

    for i in 0..(n - k + 1) {
        let low = i;
        let high = i + k - 1;
        if x_list[high].abs() < x_list[low].abs() {
            if x_list[high].abs() + x_list[high] - x_list[low] < cost {
                cost = x_list[high].abs() + x_list[high] - x_list[low];
            }
        } else {
            if x_list[low].abs() + x_list[high] - x_list[low] < cost {
                cost = x_list[low].abs() + x_list[high] - x_list[low];
            }
        }
    }

    println!("{}", cost);
}
