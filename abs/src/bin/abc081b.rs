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
        a_lst: [usize; n],
    }

    let mut a_lst = a_lst;
    let mut count = 0;
    loop {
        if a_lst.iter().any(|a| a % 2 == 1) {
            break;
        }
        a_lst = a_lst.iter().map(|a| a / 2).collect();
        count += 1;
    }

    println!("{}", count);
}
