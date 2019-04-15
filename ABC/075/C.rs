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
    let (n, m) = input!(usize, usize);
    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(m);
    let mut graph: Vec<Vec<bool>> = vec![vec![false; n+1]; n+1];
    for _ in 0..m {
        let (from, to) = input!(usize, usize);
        graph[from][to] = true;
        graph[to][from] = true;
        edges.push((from, to));
    }

    let mut cnt = 0;
    for &(from, to) in edges.iter() {
        graph[from][to] = false;
        graph[to][from] = false;
        let mut trail: Vec<bool> = vec![false; n+1];
        let mut stack: Vec<usize> = vec![1];
        while let Some(edge_from) = stack.pop() {
            trail[edge_from] = true;
            for (edge_to, &is_exits_path) in graph[edge_from].iter().enumerate() {
                if is_exits_path == true && trail[edge_to] != true && stack.contains(&edge_to) == false {
                    stack.push(edge_to);
                }
            }
        }
        if trail.iter().skip(1).any(|&x| x == false) {
            cnt += 1;
        }
        graph[from][to] = true;
        graph[to][from] = true;
    }
    println!("{}", cnt);
}
