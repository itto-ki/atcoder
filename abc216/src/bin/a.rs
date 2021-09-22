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
        xy: f32,
    }

    let xy_10 = (xy * 10f32) as i32;
    if 0 <= xy_10 % 10 && xy_10 % 10 <= 2 {
        println!("{}-", xy_10 / 10);
    } else if 3 <= xy_10 % 10 && xy_10 % 10 <= 6 {
        println!("{}", xy_10 / 10);
    } else {
        println!("{}+", xy_10 / 10);
    }
}
