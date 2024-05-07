#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

fn check(p: (usize, usize), d: usize, h: usize, w: usize, graph: &[Vec<char>]) -> bool {
  for (dx, dy) in &[(d, d), (d, !d + 1), (!d + 1, d), (!d + 1, !d + 1)] {
    let (x, y) = (p.0.wrapping_add(*dx), p.1.wrapping_add(*dy));
    if x >= w || y >= h || graph[y][x] == '.' {
      return false;
    }
  }
  true
}

fn main() {
  input! {
    h: usize,
    w: usize,
    graph: [Chars; h],
  }

  let mut ans = vec![0; min(h, w)];

  for y in 0..h {
    for x in 0..w {
      if graph[y][x] == '#' && check((x, y), 1, h, w, &graph) {
        let mut d = 1;
        while check((x, y), d + 1, h, w, &graph) {
          d += 1;
        }

        ans[d - 1] += 1;
      }
    }
  }

  println!("{}", ans.iter().join(" "));
}
