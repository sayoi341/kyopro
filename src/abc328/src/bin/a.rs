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
        x: usize,
        sa: [usize;n]
    }

    println!("{}", sa.iter().filter(|&a| a <= &x).sum::<usize>())
}
