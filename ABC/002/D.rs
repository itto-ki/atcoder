const N_MAX: usize = 12;

fn i32_to_bitvec(integer: i32) -> Vec<bool> {
    let mut v: Vec<bool> = Vec::new();
    for i in 0..N_MAX {
        if integer & (1 << i) > 0 {
            v.push(true);
        } else {
            v.push(false);
        }
    }
    v
}

#[allow(unused_must_use)]
fn read_usize_pair() -> (usize, usize) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let v: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn main() {
    let (n, m) = read_usize_pair();
    let mut relations = [[false; N_MAX]; N_MAX];
    for _ in 0..m {
        let (x, y) = read_usize_pair();
        relations[x-1][y-1] = true;
        relations[y-1][x-1] = true;
    }
    let mut answer = 0;
    'nextx: for x in 0..(1 << n) {
        let bitvector = i32_to_bitvec(x);
        let bool_to_i32 = |x: bool| -> i32 { if x { 1 } else { 0 } };
        let number_of_1bit = bitvector.iter().fold(0, |acc, &x| acc + bool_to_i32(x));
        if number_of_1bit <= answer { continue; }
        for i in 0..n {
            for j in 0..i {
                if bitvector[i] && bitvector[j] && !relations[i][j] {
                    continue 'nextx;
                }
            }
        }
        answer = number_of_1bit;
    }
    println!("{}", answer);
}
