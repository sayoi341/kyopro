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
        mut n: Chars,
    }

    // 二進数nを10進数表記で出力する

    let mut ans = 0;
    let mut i = 0;

    while !n.is_empty() {
        if let Some('1') = n.pop() {
            ans += 2i64.pow(i)
        }
        i += 1;
    }

    println!("{}", ans);
}
