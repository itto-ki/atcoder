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
    let mut abc = input!(i32; 3);
    let mut cnt = 0;

    while !(abc[0] == abc[1] && abc[1] == abc[2]) {
        let mut min_i = vec![0];
        for i in 1..3 {
            if abc[min_i[0]] > abc[i] {
                min_i.pop();
                min_i.push(i);
            } else if abc[min_i[0]] == abc[i] {
                min_i.push(i);
            }
        }
        if min_i.len() == 1 {
            abc[min_i[0]] += 2;
        } else {
            abc[min_i[0]] += 1;
            abc[min_i[1]] += 1;
        }
        cnt += 1;
    }
    println!("{}", cnt);
}
