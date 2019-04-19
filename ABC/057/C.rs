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

fn calculate_digit(x: i64) -> u32 {
    let mut i = 1;
    while x >= 10_i64.pow(i) {
        i += 1;
    }
    i
}


fn f(a: i64, b: i64) -> u32 {
    let a_digit = calculate_digit(a);
    let b_digit = calculate_digit(b);
    if a_digit > b_digit { a_digit } else { b_digit }
}


fn main() {
    let n = input!(i64);

    let mut min_digit = u32::max_value();
    let mut a = 1;
    while a * a <= n {
        if n % a == 0 {
            let digit = f(a, n / a);
            if min_digit > digit {
                min_digit = digit;
            }
        }
        a += 1;
    }
    println!("{}", min_digit);
}
