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
        k: usize,
        parr: [usize; n],
        qarr: [usize; n],
    }

    for p in &parr {
        for q in &qarr {
            if p + q == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
