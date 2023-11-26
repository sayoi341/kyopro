#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use proconio::{input, marker::*};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::*,
    iter::{FromIterator, IntoIterator},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
    vec,
};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aa: [usize; n],
    }

    aa.sort();
    let mut ans = 0;

    for i in 0..n {
        let x = aa[i];
        // x + m 以上の値が何番目にあるかを二分探索
        let cnt = aa.binary_search(&(x + m)).unwrap_or_else(|e| e);
        ans = max(ans, cnt - i);
    }

    println!("{}", ans);
}
