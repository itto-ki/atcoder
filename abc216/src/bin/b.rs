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
        n: usize,
        names: [String; 2 * n],
    }

    let mut hset = HashSet::new();
    let mut i = 0;
    while i < 2 * n {
        if !hset.contains(&format!("{} {}", names[i], names[i + 1])) {
            hset.insert(format!("{} {}", names[i], names[i + 1]));
            i += 2;
        } else {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
