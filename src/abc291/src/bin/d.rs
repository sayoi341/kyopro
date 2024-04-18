#![allow(unused_imports)]
use ac_library::modint::{ModInt1000000007, ModInt998244353};
use ac_library::Dsu;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::*};
use std::vec;
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
        ab: [(usize, usize); n]
    }

    let mut dp: Vec<(usize, usize)> = Vec::new();
    dp.push((1, 1));

    for i in 1..n {
        dp.push((0, 0));

        //a to a
        if ab[i].0 != ab[i - 1].0 {
            dp[i].0 += dp[i - 1].0;
        }

        //a to b
        if ab[i].0 != ab[i - 1].1 {
            dp[i].0 += dp[i - 1].1;
        }

        //b to a
        if ab[i].1 != ab[i - 1].0 {
            dp[i].1 += dp[i - 1].0;
        }

        //b to b
        if ab[i].1 != ab[i - 1].1 {
            dp[i].1 += dp[i - 1].1;
        }

        dp[i].0 %= 998244353;
        dp[i].1 %= 998244353;
    }

    println!("{}", (dp[n - 1].0 + dp[n - 1].1) % 998244353);
}
