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
    let (h, w) = input!(i32, i32);
    let mut s = Vec::<Vec<char>>::with_capacity(h as usize);
    for _ in 0..h {
        s.push(input!(String).chars().collect());
    }

    for i in 0..h {
        for j in 0..w {
            let up_and_down    = [-1, 0, 1, 0];
            let right_and_left = [0, -1, 0, 1];
            if s[i as usize][j as usize] == '.' {
                continue;
            }
            let mut black = false;
            for k in 0..4 {
                if (i as i32) + up_and_down[k] >= 0 && (i as i32) + up_and_down[k] < h {
                    if (j as i32) + right_and_left[k] >= 0 && (j as i32) + right_and_left[k] < w {
                        if s[(i + up_and_down[k]) as usize][(j + right_and_left[k]) as usize] == '#' {
                            black = true;
                        }
                    }
                }
            }
            if !black {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
