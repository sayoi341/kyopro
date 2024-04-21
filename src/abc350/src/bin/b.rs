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
    t_arr: [usize; q],
  }

  let mut tooth_set: BTreeSet<usize> = BTreeSet::new();

  for t in t_arr {
    if tooth_set.contains(&t) {
      tooth_set.remove(&t);
    } else {
      tooth_set.insert(t);
    }
  }

  println!("{}", n - tooth_set.len());
}
