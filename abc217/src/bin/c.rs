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
        ps: [usize; n],
    }

    let mut qs = vec![String::new(); n];

    for (i, p) in ps.iter().enumerate() {
        qs[p - 1] = format!("{}", i + 1);
    }

    println!("{}", qs.join(" "));
}
