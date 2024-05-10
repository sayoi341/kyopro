#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};

fn main() {
  input! {
    n1: usize,
    n2: usize,
    m: usize,
    tmpg: [(Usize1, Usize1); m],
  }

  let mut graph = vec![vec![]; n1 + n2];

  for (a, b) in tmpg {
    graph[a].push(b);
    graph[b].push(a);
  }

  let mut dist1 = vec![-1; n1 + n2];
  let mut dist2 = vec![-1; n1 + n2];

  let mut queue1 = VecDeque::new();
  let mut queue2 = VecDeque::new();

  dist1[0] = 0;
  queue1.push_back(0);

  while let Some(v) = queue1.pop_front() {
    for u in &graph[v] {
      if dist1[*u] == -1 {
        dist1[*u] = dist1[v] + 1;
        queue1.push_back(*u);
      }
    }
  }

  dist2[n1 + n2 - 1] = 0;
  queue2.push_back(n1 + n2 - 1);

  while let Some(v) = queue2.pop_front() {
    for u in &graph[v] {
      if dist2[*u] == -1 {
        dist2[*u] = dist2[v] + 1;
        queue2.push_back(*u);
      }
    }
  }

  println!("{}", dist1.iter().max().unwrap() + dist2.iter().max().unwrap() + 1);
}
