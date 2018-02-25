fn read_i32() -> i32 {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn read_i32vec() -> Vec<i32> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let _ = read_i32();
    let a_vec = read_i32vec();
    let mut total_bugs = 0;
    let mut bug_softwares = 0;
    for i in 0..a_vec.len() {
        if a_vec[i] != 0 {
            total_bugs += a_vec[i];
            bug_softwares += 1;
        }
    }
    println!("{}", ((total_bugs as f32) / (bug_softwares as f32)).ceil());
}
