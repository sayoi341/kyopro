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
        mut ss: [Chars; n]
    }

    let mut ver = vec![0usize; n];
    let mut hor = vec![0usize; n];

    for i in 0..n {
        for j in 0..n {
            if ss[i][j] == 'o' {
                ver[i] += 1;
                hor[j] += 1;
            }
        }
    }

    let mut ans = 0;
    for (i, x) in ver.iter().enumerate() {
        for (j, y) in hor.iter().enumerate() {
            if *x >= 2 && *y >= 2 && ss[i][j] == 'o' {
                ans += (x - 1) * (y - 1);
            }
        }
    }

    println!("{}", ans);
}
