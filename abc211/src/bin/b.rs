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
        ss: [String; 4],
    }

    let set: HashSet<String> = vec!["H", "2B", "3B", "HR"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let input: HashSet<String> = ss.into_iter().collect();

    if set == input {
        println!("Yes");
    } else {
        println!("No");
    }
}
