#![allow(unused_imports)]
use ac_library::Dsu;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::*};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::*,
    iter::{FromIterator, IntoIterator},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        qs: [(usize, usize, usize, usize); q],
    }

    let mut sum = vec![vec![0; n + 1]; n + 1];

    for y in 0..n {
        for x in 0..n {
            sum[y][x + 1] += sum[y][x];
            if p[y][x] == 'B' {
                sum[y][x + 1] += 1;
            }
        }
    }

    for x in 0..n {
        for y in 0..n {
            sum[y + 1][x] += sum[y][x];
            if p[y][x] == 'B' {
                sum[y + 1][x] += 1;
            }
        }
    }
}
