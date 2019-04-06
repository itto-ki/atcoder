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
    const N: usize = 3;
    let mut c: Vec<Vec<i32>> = Vec::with_capacity(N);
    for _ in 0..N {
        c.push(input!(i32; N));
    }

    'outer: for a1 in 0..c[0][0]+1 {
        let mut a: Vec<i32> = Vec::with_capacity(N);
        let mut b: Vec<i32> = Vec::with_capacity(N);
        for i in 0..N {
            b.push(c[0][i] - a1);
            a.push(c[i][0] - b[0]);
        }
        for i in 1..N {
            for j in 1..N {
                if c[i][j] != a[i] + b[j] {
                    continue 'outer;
                }
            }
        }
        println!("Yes");
        return;
    }
    println!("No");
}
