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
        aa: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut acc = vec![0; n + 1];

    for i in 0..n {
        acc[i + 1] = acc[i];
        if aa[i] == 0 {
            acc[i + 1] -= 1;
        } else {
            acc[i + 1] += 1;
        }
    }

    for (l, r) in lr {
        let ans = acc[r] - acc[l - 1];

        println!(
            "{}",
            match ans {
                0 => "draw",
                _ if ans > 0 => "win",
                _ => "lose",
            }
        );
    }
}
