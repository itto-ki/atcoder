fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let s = read_string();
    let mut chars = s.chars();
    let mut rslt = String::new();
    let mut cnt: usize = 0;
    let mut past: Option<char> = None;
    loop {
        let now = chars.next();
        if now == None {
            rslt.push_str(&format!("{}{}", past.unwrap(), cnt));
            break;
        } else if past == None {
            past = now;
            cnt += 1;
        } else {
            if now == past {
                cnt += 1;
            } else {
                rslt.push_str(&format!("{}{}", past.unwrap(), cnt));
                past = now;
                cnt = 1;
            }
        }
    }
    println!("{}", rslt);
}
