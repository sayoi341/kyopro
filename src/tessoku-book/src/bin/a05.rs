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
        n: isize,
        k: isize,
    }

    let mut ans: usize = 0;

    for i in 1..=n {
        for j in 1..=n {
            if k - i - j <= n && k - i - j >= 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
