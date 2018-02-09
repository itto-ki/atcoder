const N: usize = 4;

#[allow(unused_must_use)]
fn read_stringvector() -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let mut v: Vec<Vec<String>> = Vec::with_capacity(N);
    for _ in 0..N {
        let mut line = read_stringvector();
        line.reverse();
        v.push(line);
    }
    v.reverse();
    for line in v.iter() {
        for row in line.iter() {
            print!("{} ", row);
        }
        println!("");
    }
}
