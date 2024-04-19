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
    m: usize,
    edges: [(Usize1, Usize1); m]
  }

  // unionfindの作成
  let mut dsu = Dsu::new(n);

  // 頂点をunionfindで分類
  for &(i, j) in &edges {
    dsu.merge(i, j);
  }

  // どの連結成分に含まれるか
  let mut group_id = vec![0usize; n];
  for (i, g) in group_id.iter_mut().enumerate() {
    *g = dsu.leader(i);
  }

  // 連結成分の頂点数を数える
  let mut group_size = vec![0usize; n];
  for g in &group_id {
    group_size[*g] += 1;
  }

  // 連結成分の辺数を数える
  let mut group_edge = vec![0usize; n];
  for &(u, _v) in &edges {
    group_edge[group_id[u]] += 1;
  }

  // 全ての連結成分の頂点数と辺数が一致しているか
  let ans = group_size.iter().zip(group_edge.iter()).all(|(s, e)| *s == *e);
  println!("{}", if ans { "Yes" } else { "No" });
}
