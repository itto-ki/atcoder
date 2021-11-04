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
        a: u64,
        b: u64,
        c: u64,
        x: u64,
    }

    let mut count = 0;
    for p in 0..=a {
        for q in 0..=b {
            for r in 0..=c {
                if 500 * p + 100 * q + 50 * r == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
