fn read_usize() -> usize {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn read_usizepair() -> (usize, usize) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn read_usizevec() -> Vec<usize> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let n = read_usize();
    let (a, b) = read_usizepair();
    let _ = read_usize();
    let ps = read_usizevec();

    let mut flags = vec![false; n];
    flags[a-1] = true;
    flags[b-1] = true;
    for p in ps.iter() {
        if flags[p-1] == true {
            println!("NO");
            return;
        } else {
            flags[p-1] = true;
        }
    }
    println!("YES");
}
