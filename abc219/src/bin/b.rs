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
        s: [String; 3],
        t: String,
    }

    for c in t.chars() {
        print!("{}", s[c.to_digit(10).unwrap() as usize - 1]);
    }
}
