#[allow(unused_must_use)]
fn read_intstr() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn main() {
    let n = read_intstr();
    if n.contains("3") || n.parse::<i32>().unwrap() % 3 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
