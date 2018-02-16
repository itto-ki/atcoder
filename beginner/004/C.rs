const N_MAX: usize = 30;

#[allow(unused_must_use)]
fn read_usize() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn swap(i: usize) -> Vec<i32> {
    let mut v = vec!(1, 2, 3, 4, 5, 6);
    for c in 0..i+1 {
        let d = (c % 5) as usize;
        v[d] = v[d] ^ v[d+1];
        v[d+1] = v[d] ^ v[d+1];
        v[d] = v[d] ^ v[d+1];
    }
    v
}

fn main() {
    let n = read_usize();
    let mut v = Vec::with_capacity(N_MAX);
    for i in 0..N_MAX {
        v.push(swap(i))
    }
    for x in v[(n - 1) % N_MAX].iter() {
        print!("{}", x);
    }
    println!("");
}
