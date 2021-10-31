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
        (a, b, c): (u32, u32, u32),
    }

    println!("{}", vec![a + b, b + c, c + a].iter().max().unwrap());
}
