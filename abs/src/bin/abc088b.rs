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
        alist: [u64; n],
    }

    let mut alist = alist;
    alist.sort();
    let alist = alist.into_iter().rev().collect::<Vec<u64>>();
    let alice = alist
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, x)| x)
        .sum::<u64>();
    let bob = alist
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 1)
        .map(|(_, x)| x)
        .sum::<u64>();
    println!("{}", alice - bob);
}
