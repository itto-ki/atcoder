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
        (n, a, x, y): (u32, u32, u32, u32),
    }

    if n < a {
        println!("{}", x * n);
    } else {
        println!("{}", (n - a) * y + a * x);
    }
}
