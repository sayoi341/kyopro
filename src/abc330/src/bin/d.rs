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

    let mut ver = vec![0; n];
    let mut hor = vec![0; n];

    for i in 0..n - 1 {
        for j in 0..n - 1 {}
    }
}
