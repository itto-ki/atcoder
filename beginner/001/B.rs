#[allow(unused_must_use)]
fn read_int() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let m = read_int();

    if m < 100 {
        println!("00");
    }  else if m <= 5000 {
        println!("{:02}", m * 10 / 1000 as i32);
    } else if m <= 30000 {
        println!("{:02}", m / 1000 + 50 as i32);
    } else if m <= 70000 {
        println!("{:02}", (m / 1000 - 30) / 5 + 80 as i32);
    } else {
        println!("89");
    }
}
