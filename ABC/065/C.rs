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
    let (n, m) = input!(i64, i64);
    let modulo = 1000000007;

    if (n - m).abs() > 1 {
        println!("0");
        return;
    } else if (n - m).abs() == 1 {
        let n_factorial = (1..n+1).fold(1, |acc, x| (acc * x) % modulo);
        let m_factorial = (1..m+1).fold(1, |acc, x| (acc * x) % modulo);
        println!("{}", (n_factorial * m_factorial) % modulo);
    } else {
        let n_factorial = (1..n+1).fold(1, |acc, x| (acc * x) % modulo);
        let m_factorial = (1..m+1).fold(1, |acc, x| (acc * x) % modulo);
        println!("{}", 2 * ((n_factorial * m_factorial) % modulo) % modulo);
    }
}
