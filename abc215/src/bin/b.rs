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
        n: i64,
    }

    let mut i = 0;
    loop {
        if 2_i64.pow(i) > n {
            break;
        }
        i += 1;
    }

    println!("{}", i - 1);
}
