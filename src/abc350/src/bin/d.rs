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
  vec,
};
use superslice::Ext;

#[fastout]
fn main() {
  input! {
    n: usize,
    m: isize,
    ab_arr: [(Usize1, Usize1); m],
  }

  let mut uf = Dsu::new(n);
  for (a, b) in ab_arr.clone() {
    uf.merge(a, b);
  }

  let mut leader = vec![0; n];
  for (i, val) in leader.iter_mut().enumerate() {
    *val = uf.leader(i);
  }

  let mut leader_count = vec![0; n];
  for &l in &leader {
    leader_count[l] += 1;
  }

  let mut ans = -m;

  for i in leader_count {
    ans += (i * (i - 1)) / 2;
  }

  println!("{}", ans);
}
