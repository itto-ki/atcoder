#[allow(unused_must_use)]
fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let s = read_string();
    let t = read_string();
    for i in 0..s.len() {
        if s.chars().nth(i) != t.chars().nth(i) {
            match s.chars().nth(i).unwrap() {
                '@' => {    // s[i]が'@'ならば
                    match t.chars().nth(i).unwrap() {
                        'a' | 't' | 'c' | 'o' | 'd' | 'e' | 'r' => continue,    // t[i]が'atcoder'であれば良い
                        _ => { println!("You will lose"); return; },
                    }
                },
                'a' | 't' | 'c' | 'o' | 'd' | 'e' | 'r' => {    // s[i]が'atcoder'ならば
                    if t.chars().nth(i).unwrap() == '@' {    // t[i]が'@'であれば良い
                        continue;
                    } else {
                        println!("You will lose");
                        return;
                    }
                },
                _ => { println!("You will lose"); return; },    // s[i]が'@'でも'atcoder'でもなければ修正不可
            }
        }
    }
    println!("You can win");
}
