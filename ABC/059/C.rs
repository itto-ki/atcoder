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

fn counter(a: &mut Vec<i64>, judge: &Fn(usize) -> bool) -> i64 {
    let mut sum = 0;
    let mut counter = 0;
    for i in 0..a.len() {
        if judge(i) {
            if sum + a[i] == 0 {
                a[i] += 1;
                counter += 1;
            } else if sum + a[i] < 0 {
                counter += (sum + a[i]).abs() + 1;
                a[i] += (sum + a[i]).abs() + 1;
            }
        } else {
            if sum + a[i] == 0 {
                a[i] -= 1;
                counter += 1;
            } else if sum + a[i] > 0 {
                counter += (sum + a[i] + 1).abs();
                a[i] -= sum + a[i] + 1;
            }
        }
        sum += a[i];
    }
    counter
}

fn main() {
    let _n = input!(usize);
    let a = input!(i64; n);

    let answer1 = counter(&mut a.clone(), &|x| x % 2 == 0);
    let answer2 = counter(&mut a.clone(), &|x| x % 2 != 0);
    println!("{}", if answer1 < answer2 { answer1 } else { answer2 });
}
