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
        ss:Chars,
        ts:Chars,
    }

    let mut i = 0usize;

    for s in ss {
        if s == ts[i].to_lowercase().to_string().chars().next().unwrap() {
            i += 1;

            if i == 3 {
                println!("Yes");
                return;
            }

            if i == 2 && ts[2] == 'X' {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
