macro_rules! input {
    ($ttype:ty) => {
        {
            let mut input = String::new();
            let _= std::io::stdin().read_line(&mut input);
            input.trim().parse::<$ttype>().unwrap()
        }
    };

    ($($ttype:ty),*) => {
        {
            use std::io::BufRead;
            let mut input = String::new();
            let stdin = std::io::stdin();
            let _ = stdin.lock().read_line(&mut input);
            let mut iter = input.trim().split_whitespace();
            (
                $(iter.next().unwrap().parse::<$ttype>().unwrap(),)*
            )
        }
    };

    ($ttype:ty; $x:expr) => {
        {
            use std::io::BufRead;
            let mut input = String::new();
            let stdin = std::io::stdin();
            let _ = stdin.lock().read_line(&mut input);
            let v: Vec<$ttype> = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            v
        }
    };

    ($ttype:ty; $x:expr; $y:expr) => {
        {
            use std::io::BufRead;
            let mut matrix = Vec::with_capacity($y);
            for _ in 0..$y {
                let mut input = String::new();
                let stdin = std::io::stdin();
                let _ = stdin.lock().read_line(&mut input);
                let v: Vec<$ttype> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                matrix.push(v);
            }
            matrix
        }
    };
}

use std::collections::HashMap;

fn main() {
    let _n = input!(usize);
    let a_lst = input!(i64; n);
    let mut map: HashMap<i64, i64> = HashMap::new();

    for a in a_lst.iter() {
        if map.contains_key(a) {
            if let Some(x) = map.get_mut(a) {
                *x += 1;
            }
        } else {
            map.insert(*a, 1);
        }
    }

    let mut long_1st: i64 = 0;
    let mut long_2nd: i64 = 0;
    for (&key, &value) in map.iter() {
        if value >= 2 {
            if value / 2 < 1 {
                break;
            }
            if value / 2 > 1 {
                if key > long_1st {
                    long_2nd = long_1st;
                    long_1st = key;
                } else if key > long_2nd {
                    long_2nd = key;
                }
            }
            if key > long_1st {
                long_2nd = long_1st;
                long_1st = key;
            } else if key > long_2nd {
                long_2nd = key;
            }
        }
    }
    println!("{}", long_1st * long_2nd);
}
