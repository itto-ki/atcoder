#[allow(unused_must_use)]
fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let mut a = read_string();
    if a.len() == 1 {
        if a == "a" {
            println!("-1");
        } else {
            println!("{}", (a.as_bytes()[0] -1) as char);
        }
    } else {
        a.truncate(1);
        println!("{}", a);
    }
}
