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
    let (a, b, c, d, e, f) = input!(i64, i64, i64, i64, i64, i64);

    let mut max_density = 0.0;
    let mut max_sugarwater = 0;
    let mut max_sugar = 0;
    for water_a in 0..f {
        for water_b in 0..f {
            let water = 100 * (water_a * a + water_b * b);
            if water > f { break; }
            for sugar_c in 0..f {
                for sugar_d in 0..f {
                    let sugar = c * sugar_c + d * sugar_d;
                    if sugar + water > f  || sugar > e * water / 100 {
                        break;
                    } else {
                        let density = 100.0 * sugar as f64 / (water as f64 + sugar as f64);
                        if density >= max_density {
                            max_density = density;
                            max_sugarwater = sugar + water;
                            max_sugar = sugar;
                        }
                    }
                }
            }
        }
    }
    println!("{} {}", max_sugarwater, max_sugar);
}
