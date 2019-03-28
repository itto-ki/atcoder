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
    let N: usize = 300000;
    let mut east = vec![0; N];
    let mut east_cnt = 0;
    let mut west = vec![0; N];
    let mut west_cnt = 0;
    let mut sum = 0;
    let mut max = std::i32::MIN;
    let mut index: usize = 0;
    let n = input!(usize);
    let s = input!(String);
    for (i, c) in s.chars().enumerate() {
        if c == 'E' { 
            east_cnt += 1;
            east[i] = east_cnt;
            west[i] = west_cnt;
            sum += 1;
        } else {
            east[i] = east_cnt;
            west_cnt += 1;
            west[i] = west_cnt;
            sum -= 1;
        }
        if sum > max {
            max = sum;
            index = i;
        }
    }

    if index < 1 {
        println!("{}", east[n-1] - east[index+1]);
    } else if index + 1 >= N {
        println!("{}", west[index-1]);
    } else {
        println!("{}", west[index-1] + (east[n-1] - east[index]));
    }
}
