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

fn dfs(graph: &Vec<Vec<usize>>, v: (usize, usize), mut visited_num: BTreeSet<usize>, dist: &mut Vec<Vec<usize>>, w: usize, h: usize) {
  //現在の位置の数字を訪問済みにする
  visited_num.insert(graph[v.0][v.1]);
  dist[v.0][v.1] += 1;

  for (x, y) in [(0usize, 1usize), (1usize, 0usize)] {
    if v.0 + x > h || v.1 + y > w {
      continue;
    }

    let next = graph[v.0 + x][v.1 + y];

    if !visited_num.contains(&next) {
      dfs(graph, (v.0 + x, v.1 + y), visited_num.clone(), dist, w, h);
    }
  }
}

#[fastout]
fn main() {
  input! {
    h: Usize1,
    w: Usize1,
    graph: [[usize; w + 1]; h + 1]
  }

  let visited_num = BTreeSet::new();
  let mut dist = vec![vec![0; w + 1]; h + 1];

  dfs(&graph, (0, 0), visited_num.clone(), &mut dist, w, h);

  println!("{}", dist[h][w]);
}
