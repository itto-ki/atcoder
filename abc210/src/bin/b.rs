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
        _n: usize,
        s: Chars,
    }

    for (i, c) in s.into_iter().enumerate() {
        if c == '1' {
            if i % 2 == 0 {
                println!("Takahashi");
                return;
            } else {
                println!("Aoki");
                return;
            }
        }
    }
}
