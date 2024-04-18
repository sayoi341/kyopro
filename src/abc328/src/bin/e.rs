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
      m: usize,
      k: usize,
      edges: [(Usize1, Usize1, usize); m],
  }

  let mut ans = usize::MAX;

  // 辺の組み合わせを全探索
  for edge in edges.iter().combinations_with_replacement(n - 1) {
    let mut uf = Dsu::new(n);
    let mut weight = 0;

    // 辺の重みを計算
    // unionfindで連結成分を計算
    for &(u, v, w) in edge {
      uf.merge(u, v);
      weight = (weight + w) % k;
    }

    // 連結成分が1つの場合のみ、重みを更新
    if uf.size(0) == n {
      ans = ans.min(weight);
    }
  }

  println!("{}", ans);
}
