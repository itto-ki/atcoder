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
        (n, x): (usize, i32),
        a_lst: [i32; n],
    }

    let mut sum = 0;

    for (i, a) in a_lst.iter().enumerate() {
        if i % 2 != 0 {
            sum += a - 1;
        } else {
            sum += a;
        }
    }

    if sum > x {
        println!("No");
    } else {
        println!("Yes");
    }
}
