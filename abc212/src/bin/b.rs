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
        xs: Chars,
    }

    // 全一致
    if xs.iter().all(|x| xs[0] == *x) {
        println!("Weak");
        return;
    }

    // x = x+1
    let mut is_weak = true;
    for i in 0..3 {
        if (xs[i].to_digit(10).unwrap() + 1) % 10 != xs[i + 1].to_digit(10).unwrap() {
            is_weak = false;
        }
    }

    if is_weak {
        println!("Weak");
    } else {
        println!("Strong");
    }
}
