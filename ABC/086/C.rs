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
    let mut lst: Vec<(i32, i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        lst.push(input!(i32, i32, i32));
    }

    let mut now: (i32, i32, i32) = (0, 0, 0);

    for next in lst.iter() {
        let delta_t = next.0 - now.0;
        let delta_x = (next.1 - now.1).abs();
        let delta_y = (next.2 - now.2).abs();
        if delta_x + delta_y > delta_t || (delta_x + delta_y - delta_t) % 2 != 0 {
            println!("No");
            return;
        }
        now = *next;
    }
    println!("Yes");
}
