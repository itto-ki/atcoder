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
        (n, a, b): (u64, u64, u64),
    }

    let mut sum = 0;
    for x in 1..n + 1 {
        let mut y = x;
        let mut digit_sum = 0;
        loop {
            digit_sum += y % 10;
            y /= 10;
            if y < 1 {
                break;
            }
        }
        if a <= digit_sum && digit_sum <= b {
            sum += x;
        }
    }

    println!("{}", sum);
}
