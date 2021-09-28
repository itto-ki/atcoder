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
        a: [usize; n],
    }

    let mut b = a.clone();
    b.sort();
    let b: Vec<&usize> = b.iter().rev().collect();

    for (i, x) in a.iter().enumerate() {
        if b[1] == x {
            println!("{}", i + 1);
            break;
        }
    }
}
