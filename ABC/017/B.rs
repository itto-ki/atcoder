fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let x = read_string();
    let mut chars = x.chars();
    let mut flag = true;
    if x.len() != 0 {
        loop {
            match chars.next() {
                Some('c') => {
                    if chars.next() != Some('h') {
                        flag = false;
                        break;
                    }
                },
                Some('o') | Some('k') | Some('u') => {},
                None => { break; },
                _ => {
                    flag = false;
                    break;
                },
            }
        }
    }
    if flag {
        println!("YES");
    } else {
        println!("NO");
    }
}
