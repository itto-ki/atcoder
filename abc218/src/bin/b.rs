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
        ps: [u32; 26],
    }

    for p in ps {
        print!("{}", std::char::from_u32('a' as u32 + p - 1).unwrap());
    }
    println!("")
}
