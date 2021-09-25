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
        s: Chars,
        k: usize,
    }

    let result: BTreeSet<String> = (0..s.len())
        .permutations(s.len())
        .map(|x| x.iter().map(|&i| s[i]).collect())
        .collect();

    println!("{}", result.iter().nth(k - 1).unwrap());
}
