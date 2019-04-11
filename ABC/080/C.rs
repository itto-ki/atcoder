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

fn main() {
    let n = input!(usize);
    let mut f: Vec<Vec<i64>> = Vec::with_capacity(n);
    let mut p: Vec<Vec<i64>> = Vec::with_capacity(n);

    for _ in 0..n {
        f.push(input!(i64; n));
    }
    for _ in 0..n {
        p.push(input!(i64; n));
    }

    let mut max_benefit: i64 = i64::min_value();
    for i in 1..2_i32.pow(10) {    // 少なくとも1つ出店しないするため1~
        let mut benefit: i64 = 0;
        let mut counter: Vec<i64> = vec![0; n];
        for j in 0..10 {
            if i & (1 << j) > 0 {   // joisinoがjに出店しているとき
                for k in 0..n {
                    if f[k][j] == 1 {    // 店kがjにも出店していたら
                        counter[k] += 1;    // 店kとの重複出店舗数を+1
                    }
                }
            }
        }
        for (k, &cnt) in counter.iter().enumerate() {
            benefit += p[k][cnt as usize];    // 店kとcnt店舗重複しているときの利益
        }
        if benefit > max_benefit {
            max_benefit = benefit;
        }
    }
    println!("{}", max_benefit);
}
