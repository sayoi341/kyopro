#![allow(unused_imports)]
use ac_library::Dsu;
use itertools::{iproduct, Itertools, MinMaxResult};
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
      s: usize, // 6
      m: usize, // 8
      l: usize, // 12
  }

  let mut ans = usize::MAX;

  for (s_c, m_c, l_c) in iproduct!(0..=100, 0..=100, 0..100) {
    if n <= s_c * 6 + m_c * 8 + l_c * 12 {
      ans = min(ans, s_c * s + m_c * m + l_c * l);
    }
  }

  println!("{}", ans);
}
