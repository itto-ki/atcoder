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
        (a, b): (usize, usize),
    }

    if a <= b && b <= a * 6 {
        println!("Yes");
    } else {
        println!("No");
    }
}
