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
        p: usize,
    }

    let mut coins = Vec::with_capacity(10);
    let mut factorial = 1;
    for i in 0..10 {
        factorial *= i + 1;
        coins.push(factorial);
    }

    let mut remain = p;
    let mut sum = 0;
    for coin in coins.into_iter().rev() {
        if remain / coin > 0 {
            sum += remain / coin;
            remain %= coin;
        }
    }

    println!("{}", sum);
}
