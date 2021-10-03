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
        (n, m): (usize, usize),
        a: [i64; n],
        b: [i64; m],
    }

    let mut a = a;
    let mut b = b;
    a.sort();
    b.sort();

    let (mut i, mut j, mut min) = (0, 0, std::i64::MAX);
    while i < n && j < m {
        let delta_abs = (a[i] - b[j]).abs();
        if delta_abs < min {
            min = delta_abs;
        }
        if a[i] > b[j] {
            j += 1;
        } else {
            i += 1;
        }
    }

    println!("{}", min);
}
