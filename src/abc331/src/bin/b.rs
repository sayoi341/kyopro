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
        s: usize,
        m: usize,
        l: usize,
    }

    let mut vec = [(s, 6), (m, 8), (l, 12)];

    vec.sort_by(|a, b| (a.0 / a.1).cmp(&(b.0 / b.1)));

    let mut ans = usize::MAX;

    for bit in 0..(1 << vec.len()) {
        let sub_list = (0..vec.len()).filter(|&i| (bit & (1 << i)) != 0).collect::<Vec<_>>();

        println!("{:?}", sub_list);

        let mut sum = 0;
        let mut cnt = 0;
        for s in sub_list {
            let (a, b) = vec[s];
            let c: usize = n / b;
            sum += a * c;
            cnt += c * b;
            if cnt >= n {
                ans = min(ans, sum);
            }

            let c: usize = n / b + 1;

            sum += a * c;
            cnt += c * b;
            if cnt >= n {
                ans = min(ans, sum);
            }
        }
        println!("{} {}", sum, cnt);
    }

    println!("{}", ans);
}
