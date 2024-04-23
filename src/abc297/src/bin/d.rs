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
  mem::swap,
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

#[fastout]
fn main() {
  input! {
    mut a: usize,
    mut b: usize,
  }

  let mut ans = 0;

  if a < b {
    swap(&mut a, &mut b);
  }

  while b > 0 {
    ans += a / b;
    a %= b;
    swap(&mut a, &mut b);
  }

  println!("{}", ans - 1);
}
