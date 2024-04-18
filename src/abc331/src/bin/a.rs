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
      mmax: usize,
      dmax: usize,
      mut y: usize,
      mut m: usize,
      mut d: usize,
  }

  d += 1;

  if d > dmax {
    d = 1;
    m += 1;
  }

  if m > mmax {
    m = 1;
    y += 1;
  }

  println!("{} {} {}", y, m, d);
}
