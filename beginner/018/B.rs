fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

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

fn main() {
    let s = read_string();
    let mut chars: Vec<char> = s.chars().collect();
    let n = read_i32();
    for _ in 0..n {
        let (mut l, mut r) = read_usizepair();
        l -= 1;
        r -= 1;
        while l < r {
            let tmp: char = chars[l];
            chars[l] = chars[r];
            chars[r] = tmp;
            l += 1;
            r -= 1;
        }
    }
    for c in chars {
        print!("{}", c);
    }
    println!("");
}
