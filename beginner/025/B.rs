fn read_i32triple() -> (i32, i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1], v[2])
}

fn read_string_i32() -> (String, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0].to_string(), v[1].parse::<i32>().unwrap())
}

fn main() {
    let (n, a, b) = read_i32triple();

    let mut rslt: i32 = 0;
    for _ in 0..n {
        let (s, d) = read_string_i32();
        if s == "West" {
            if d < a {
                rslt += a;
            } else if d > b {
                rslt += b;
            } else {
                rslt += d;
            }
        } else {
            if d < a {
                rslt -= a;
            } else if d > b {
                rslt -= b;
            } else {
                rslt -= d;
            }
        }
    }

    if rslt > 0 {
        println!("West {}", rslt);
    } else if rslt < 0 {
        println!("East {}", -rslt);
    } else {
        println!("0");
    }
}
