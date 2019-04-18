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

fn solve(h: i64, w: i64) -> i64 {
    let mut delta = i64::max_value();
    for ah in 1..h {
        let bh = (h - ah) / 2;
        let ch = h - (ah + bh);
        let mut rectangles = vec![ah * w, bh * w, ch * w];
        rectangles.sort();
        if delta > rectangles[2] - rectangles[0] {
            delta = rectangles[2] - rectangles[0];
        }

        let bh = h - ah;
        let ch = h - ah;
        let bw = w / 2;
        let cw = w - bw;
        let mut rectangles = vec![ah * w, bh * bw, ch * cw];
        rectangles.sort();
        if delta > rectangles[2] - rectangles[0] {
            delta = rectangles[2] - rectangles[0];
        }
    }
    delta
}

fn main() {
    let (h, w) = input!(i64, i64);

    let answer1 = solve(h, w);
    let answer2 = solve(w, h);
    println!("{}", if answer1 > answer2 { answer2 } else { answer1 });
}
