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
    let abcd = input!(String);
    let numbers = abcd.chars().map(|x| x as i32 - '0' as i32).collect::<Vec<i32>>();

    for i in 0..2_i64.pow(3) {
        let mut answer = numbers[0];
        let mut operator: Vec<char> = Vec::with_capacity(3);
        for j in 0..3 {
            if i & (1 << j) > 0 {
                answer += numbers[j+1];
                operator.push('+');
            } else {
                answer -= numbers[j+1];
                operator.push('-');
            }
        }

        if answer == 7 {
            for k in 0..3 {
                if k != 2 {
                    print!("{}{}", numbers[k], operator[k]);
                } else {
                    println!("{}{}{}=7", numbers[k], operator[k], numbers[k+1]);
                }
            }
            return;
        }
    }
}
