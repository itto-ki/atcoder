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
        s: [usize; n],
        t: [usize; n],
    }

    // 高橋くんから最初に宝石を渡されるすぬけくんとその時の時間を選ぶ
    let mut min = (0, t[0]);
    for (i, x) in t.clone().into_iter().enumerate() {
        if min.1 > x {
            min = (i, x);
        }
    }

    let mut t = t;

    // 最初に宝石を渡されるすぬけくんからN番目のすぬけくんまでを計算
    for i in (min.0)..n {
        if min.0 != i {
            if t[i - 1] + s[i - 1] < t[i] {
                t[i] = t[i - 1] + s[i - 1];
            }
        }
    }

    // 1番目のすぬけくんから最初に宝石を渡されるすぬけくんまでを計算
    for i in 0..(min.0) {
        if i == 0 {
            if t[n - 1] + s[n - 1] < t[i] {
                t[i] = t[n - 1] + s[n - 1];
            }
        } else {
            if t[i - 1] + s[i - 1] < t[i] {
                t[i] = t[i - 1] + s[i - 1];
            }
        }
    }

    for x in t {
        println!("{}", x);
    }
}
