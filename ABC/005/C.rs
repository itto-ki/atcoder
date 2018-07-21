#[allow(unused_must_use)]
fn read_int() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

#[allow(unused_must_use)]
fn read_intvec() -> Vec<usize> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let t = read_int();
    let n = read_int();
    let a = read_intvec();
    let m = read_int();
    let b = read_intvec();
    if n < m {
        println!("no");
        return;
    } else {
        let mut i: usize = 0;
        for &takoyaki in a.iter() {
            if takoyaki <= b[i] && b[i] <= takoyaki + t {
                i += 1;
            }
            if i == m {
                break;
            }
        }
        if i == m {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
