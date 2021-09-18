#![allow(unused_imports)]
use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    // 各'#'の座標を取得
    let mut s_point: Vec<(i32, i32)> = Vec::new();
    let mut t_point: Vec<(i32, i32)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                s_point.push((i as i32, j as i32));
            }
            if t[i][j] == '#' {
                t_point.push((i as i32, j as i32));
            }
        }
    }

    for _ in 0..4 {
        // 一番左上に近い点の座標を取得
        let s_min = s_point.iter().min().unwrap();
        let t_min = t_point.iter().min().unwrap();

        // 平行移動
        let s_fix = s_point
            .iter()
            .map(|p| (p.0 - s_min.0, p.1 - s_min.1))
            .collect::<HashSet<(i32, i32)>>();
        let t_fix = t_point
            .iter()
            .map(|p| (p.0 - t_min.0, p.1 - t_min.1))
            .collect::<HashSet<(i32, i32)>>();
        if s_fix == t_fix {
            println!("Yes");
            return;
        }

        // 90度回転
        t_point = t_point.iter().map(|&(x, y)| (y, -x)).collect();
    }

    println!("No");
}
