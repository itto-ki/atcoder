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

fn lower_bound(b: &[i64], x: i64) -> usize {
    let (mut lower, mut upper) = (0, b.len() - 1);
    loop {
        if lower > upper {
            break;
        }
        let middle = (lower + upper) / 2;
        if b[middle] < x {
            lower = middle;
        } else {
            upper = middle - 1;
        }
    }
    lower
}

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

    let mut min = std::i64::MAX;
    for x in a.into_iter() {
        let delta = x - b[lower_bound(&b, x)];
        if delta.abs() < min {
            min = delta.abs();
        }
    }
}
