use std::collections::HashMap;

fn read_int() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let n = read_int();
    let mut candidates: HashMap<String, i32> = HashMap::new();
    for _ in 0..n {
        let candidate = read_string();
        if candidates.contains_key(&candidate) {
            if let Some(x) = candidates.get_mut(&candidate) {
                *x = *x + 1;
            }
        } else {
            candidates.insert(candidate, 1);
        }
    }
    let mut polls = 0;
    let mut leader = String::new();
    for (candidate, number) in candidates {
        if polls < number {
            polls = number;
            leader = candidate
        }
    }
    println!("{}", leader);
}
