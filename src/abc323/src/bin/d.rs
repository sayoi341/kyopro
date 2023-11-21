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
        sc: [(usize, usize); n],
    }

    let mut map = BTreeMap::new();

    // でかいスライムを分解する。
    for (s, c) in sc {
        let mut f = s;
        while f % 2 == 0 {
            f /= 2;
        }
        *map.entry(f).or_insert(0) += (s / f) * c;
    }

    let mut ans = 0;
    for (_, v) in map {
        // これやばい
        // 二進数に直した時に、1が立つbitの数を答える
        ans += v.count_ones();
    }

    println!("{}", ans);
}
