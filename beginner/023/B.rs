fn read_i32() -> i32 {
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
    let n = read_i32();
    let s = read_string();
    let mut rslt = String::from("b");
    for i in 0..100 {
        if i % 3 == 1 {
            rslt.insert(0, 'a');
            rslt.push('c');
        } else if i % 3 == 2 {
            rslt.insert(0, 'c');
            rslt.push('a');
        } else {
            if i > 0 {
                rslt.insert(0, 'b');
                rslt.push('b');
            }
        }
        if rslt.len() >= s.len() {
            if rslt == s {
                println!("{}", i);
            } else {
                println!("-1");
            }
            break;
        }
    }
}
