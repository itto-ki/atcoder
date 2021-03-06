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
    const N: usize = 100000;
    let mut dp = vec![0; N+1];
    dp[0] = 0;
    for i in 1..(N+1) {
        dp[i] = std::i32::MAX;
        let mut power = 1;
        while power <= i {
            if dp[i] > dp[i - power] + 1 {
                dp[i] = dp[i - power] + 1;
            }
            power *= 6;
        }
        power = 1;
        while power <= i {
            if dp[i] > dp[i - power] + 1 {
                dp[i] = dp[i - power] + 1;
            }
            power *= 9;
        }
    }
    let n = input!(usize);
    println!("{}", dp[n]);
}
