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
        (h, w, n): (usize, usize, usize),
        ab: [(usize, usize); n],
    }

    // 1度 HashSet とすることで重複を削除する
    let mut a = ab
        .clone()
        .into_iter()
        .map(|(a, _)| a)
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect::<Vec<usize>>();
    let mut b = ab
        .clone()
        .into_iter()
        .map(|(_, b)| b)
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect::<Vec<usize>>();

    a.sort();
    b.sort();

    // 新規に座標を割り振る
    let mut map_h = HashMap::new();
    let mut h_point = 1;
    for x in a {
        map_h.insert(x, h_point);
        h_point += 1;
    }

    let mut map_w = HashMap::new();
    let mut w_point = 1;
    for x in b {
        map_w.insert(x, w_point);
        w_point += 1;
    }

    for (a, b) in ab {
        println!("{} {}", map_h.get(&a).unwrap(), map_w.get(&b).unwrap());
    }
}
