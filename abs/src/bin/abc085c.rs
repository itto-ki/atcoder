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
        (n, y): (usize, usize),
    }

    for i in 0..=n {
        for j in 0..=(n - i) {
            if 10_000 * i + 5_000 * j + 1_000 * (n - i - j) < y {
                continue;
            }
            let k = n - i - j;
            if 10_000 * i + 5_000 * j + 1_000 * k == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
