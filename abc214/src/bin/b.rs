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
        s: i64,
        t: i64,
    }

    let mut count = 0;
    for a in 0..=100 {
        for b in 0..=(100 - a) {
            for c in 0..=(100 - (a + b)) {
                if a + b + c <= s && a * b * c <= t {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
