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
        (a, b): (u32, u32),
    }

    if b == 0 {
        println!("Gold");
    } else if a == 0 {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
