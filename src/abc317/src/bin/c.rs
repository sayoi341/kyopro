#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use petgraph::visit;
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

fn dfs(v: usize, sum: usize, graph: &[Vec<isize>], visited: &mut [bool]) -> usize {
  // println!("v: {}, sum: {}, visited: {:?}", v, sum, visited);
  visited[v] = true;
  let mut res = sum;
  for (u, c) in graph[v].iter().enumerate() {
    if visited[u] || *c == -1 {
      continue;
    }
    res = max(res, dfs(u, sum + *c as usize, graph, visited));
  }
  visited[v] = false;
  res
}

fn main() {
  input! {
    n: usize,
    m: usize,
  }

  let mut g: Vec<Vec<isize>> = vec![vec![-1; n]; n];

  for _ in 0..m {
    input! {
      a: Usize1,
      b: Usize1,
      c: isize,
    }
    g[a][b] = c;
    g[b][a] = c;
  }

  let mut ans = 0;

  for v in 0..n {
    let mut visited = vec![false; n];
    ans = max(ans, dfs(v, 0, &g, &mut visited));
  }

  println!("{}", ans);
}
