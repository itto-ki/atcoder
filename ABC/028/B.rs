fn read_string() -> String {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let s = read_string();
    let mut rslt = [0; 6];
    for c in s.chars() {
        rslt[((c as u8) - ('A' as u8)) as usize] += 1;
    }
    for (i, val) in rslt.iter().enumerate() {
        if i != 5 {
            print!("{} ", val);
        } else {
            print!("{}", val);
        }
    }
    println!("");
}
