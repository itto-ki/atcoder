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


fn update_map(key: &i64, map: &mut HashMap<i64, i64>) {
    if map.contains_key(key) {
        if let Some(x) = map.get_mut(key) {
            *x += 1;
        }
    } else {
        map.insert(*key, 1);
    }
}

fn main() {
    let _n = input!(usize);
    let a = input!(i64; n);
    let mut map: HashMap<i64, i64> = HashMap::new();

    for &a_i in a.iter() {
        update_map(&(a_i - 1), &mut map);
        update_map(&(a_i), &mut map);
        update_map(&(a_i + 1), &mut map);
    }

    let mut max = 0;
    for &value in map.values() {
        if value > max {
            max = value;
        }
    }
    println!("{}", max);
}
