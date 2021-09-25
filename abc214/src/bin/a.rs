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
    }

    if 1 <= n && n <= 125 {
        println!("4");
    } else if 126 <= n && n <= 211 {
        println!("6");
    } else {
        println!("8");
    }
}
