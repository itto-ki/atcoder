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
    }

    if !s.contains(&String::from("ABC")) {
        println!("ABC");
    } else if !s.contains(&String::from("ARC")) {
        println!("ARC");
    } else if !s.contains(&String::from("AGC")) {
        println!("AGC");
    } else {
        println!("AHC");
    }
}
