const MOD: usize = 1000000007;
const C: usize = 30;

static mut PASCAL: [[usize; C*C]; C*C] = [[0; C*C]; C*C];

#[allow(unused_must_use)]
fn read_intpair() -> (usize, usize) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let v: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

#[allow(non_snake_case)]
fn nCm_pascal() {
    unsafe {
        PASCAL[0][0] = 1;
    }
    for i in 1..C*C {
        for j in 0..C*C {
            if i == j || j == 0 {
                unsafe {
                    PASCAL[i][j] = 1;
                }
            } else {
                unsafe {
                    PASCAL[i][j] = (PASCAL[i-1][j] % MOD + PASCAL[i-1][j-1] % MOD) % MOD;
                }
            }
        }
    }
}

fn main() {
    let (r, c) = read_intpair();
    let (x, y) = read_intpair();
    #[allow(unused_variables)]
    let (d, l) = read_intpair();
    nCm_pascal();
    unsafe {
        println!("{}", PASCAL[x*y][d] * (r - x + 1) * (c - y + 1) % MOD);
    }
}
