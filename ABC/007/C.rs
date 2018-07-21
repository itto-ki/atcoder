use std::collections::VecDeque;

fn read_i32pair() -> (i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn read_vecchar() -> Vec<char> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().chars().collect()
}

fn main() {
    let (r, c) = read_i32pair();
    let (sy, sx) = read_i32pair();
    let (gy, gx) = read_i32pair();
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut maze: Vec<Vec<char>> = Vec::with_capacity(r as usize);
    let mut depth_of_maze: Vec<Vec<i32>> = Vec::with_capacity(r as usize);
    for _ in 0..r {
        maze.push(read_vecchar());
        depth_of_maze.push(vec![-1; c as usize]);
    }

    let mut depth_of_goal: u32 = 0;
    depth_of_maze[(sy - 1) as usize][(sx - 1) as usize] = 0;
    queue.push_back((sy - 1, sx - 1));
    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        let depth = depth_of_maze[y as usize][x as usize];
        for (ny, nx) in [-1, 1, 0, 0].iter().zip([0, 0, -1, 1].iter()) {
            if maze[(y + ny) as usize][(x + nx) as usize] == '.' {
                // 訪れた箇所は'x'とする
                maze[(y + ny) as usize][(x + nx) as usize] = 'x';
                depth_of_maze[(y + ny) as usize][(x + nx) as usize] = depth + 1;
                queue.push_back((y + ny, x + nx));
            }
        }
    }
    println!("{}", depth_of_maze[(gy - 1) as usize][(gx - 1) as usize]);
}
