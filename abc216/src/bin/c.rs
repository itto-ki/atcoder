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
    }

    let mut n0 = n;
    let mut result = String::new();

    // 問題設定とは逆に計算していく
    loop {
        // 残り1個になったら-1して終了
        if n0 == 1 {
            result.push('A');
            break;
        }

        // 残りが奇数なら-1
        if n0 % 2 == 1 {
            result.push('A');
            n0 -= 1
        }

        // 上記計算で偶数個になっているはずなので、2分の1する
        n0 /= 2;
        result.push('B');
    }

    println!("{}", result.chars().rev().collect::<String>());
}
