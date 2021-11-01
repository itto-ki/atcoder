#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input!{
        (a, b, c, d): (f64, f64, f64, f64),
    }

    if c * d - b <= 0.0 {
        println!("-1");
        return;
    }

    println!("{}", (a / (c * d - b)).ceil() as u64);
}
