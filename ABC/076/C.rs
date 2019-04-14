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
    let s = input!(String);
    let t = input!(String);

    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();

    if s.len() < t.len() {
        println!("UNRESTORABLE");
        return;
    }

    for i in (0..(s.len() - t.len() + 1)).rev() {
        for j in (0..t.len()).rev() {
            if s[i+j] == t[j] || s[i+j] == '?' {
                if j == 0 {
                    for k in 0..s.len() {
                        if k < i || i + t.len() <= k {
                            if s[k] == '?' {
                                print!("a");
                            } else {
                                print!("{}", s[k]);
                            }
                        } else {
                            print!("{}", t[k-i]);
                        }
                    }
                    println!("");
                    return;
                }
            } else {
                break;
            }
        }
    }
    println!("UNRESTORABLE");
}
