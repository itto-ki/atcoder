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

fn countup_target<T: PartialEq>(target: T, vec: Vec<T>) -> usize {
    let mut counter: usize = 0;
    for item in vec {
        if target == item {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let n = input!(usize);
    let mut s_list: Vec<Vec<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        let s = input!(String);
        s_list.push(s.chars().collect());
    }

    s_list.sort_by_key(|x| x.len());
    let mut alphabet_counter: Vec<(_, _)> = Vec::with_capacity(26);
    for alphabet in ('a' as u8)..('z' as u8)+1 {
        let mut counter = usize::max_value();
        for s in s_list.iter() {
            let cnt = countup_target(alphabet as char, s.to_vec());
            if counter > cnt {
                counter = cnt;
            }
        }
        alphabet_counter.push((alphabet as char, counter));
    }

    for &(alphabet, counter) in alphabet_counter.iter() {
        for _ in 0..counter {
            print!("{}", alphabet);
        }
    }
    println!("");
}
