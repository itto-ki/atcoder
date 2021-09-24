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
        x: String,
        n: usize,
        ss: [String; n],
    }

    let mut dict = HashMap::new();
    for (i, c) in x.chars().enumerate() {
        // +1をしないと、文字列がそこで終わっているのか、それともインデックス0の文字なのかが判定できずにおかしくなる
        dict.insert(c, i + 1);
    }

    let mut orders = Vec::with_capacity(n);
    for (i, s) in ss.iter().enumerate() {
        let mut order = vec![0usize; 10];
        for (j, c) in s.chars().enumerate() {
            order[j] = *dict.get(&c).unwrap();
        }
        orders.push((order, i));
    }

    orders.sort();

    for (_, i) in orders.iter() {
        println!("{}", ss[*i]);
    }
}
