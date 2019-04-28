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

fn calulate_unvisited_neighbors(now: usize, edges: &Vec<Vec<bool>>, visited: &Vec<usize>) -> Vec<usize> {
    let mut neighbors: Vec<usize> = Vec::new();
    for (neighbor, &edge) in edges[now].iter().enumerate() {
        if edge && !visited.contains(&neighbor) {
            neighbors.push(neighbor);
        }
    }
    neighbors
}

fn traversal(now: usize, edges: &Vec<Vec<bool>>, visited: &Vec<usize>) -> i64 {
    let mut visited = visited.clone();
    visited.push(now);
    let neighbors = calulate_unvisited_neighbors(now, edges, &visited);
    let mut counter = 0;
    if neighbors.is_empty() {
        if visited.len() == edges.len() - 1 {
            return 1
        } else {
            return 0
        }
    }
    for &neighbor in neighbors.iter() {
        let now = neighbor;
        counter += traversal(now, edges, &visited);
    }
    counter
}

fn main() {
    let (n, m) = input!(usize, usize);
    let mut edges = vec![vec![false; n+1]; n+1];
    for _ in 0..m {
        let (a, b) = input!(usize, usize);
        edges[a][b] = true;
        edges[b][a] = true;
    }
    println!("{:?}", traversal(1, &edges, &vec![]));
}
