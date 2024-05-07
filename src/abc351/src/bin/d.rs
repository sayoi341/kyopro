#![allow(unused_imports)]
use ac_library::*;
use itertools::{iproduct, Itertools, MinMaxResult};
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

fn main() {
  input! {
    h: usize,
    w: usize,
    mut s: [Chars; h],
  }

  let mut dist = vec![vec![0; w]; h];
  let mut visited = vec![vec![false; w]; h];

  for y in 0..h {
    for x in 0..w {
      let c = s[y][x];
      if c == '#' {
        dist[y][x] = 0;
        visited[y][x] = true;
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
          let x = x as i32 + dx;
          let y = y as i32 + dy;
          if x < 0 || y < 0 || y >= h as i32 || x >= w as i32 {
            continue;
          }
          if s[y as usize][x as usize] == '.' {
            s[y as usize][x as usize] = '+';
            dist[y as usize][x as usize] = 1;
            visited[y as usize][x as usize] = true;
          }
        }
      }
    }
  }

  // distの最大値を求める
  let mut max_dist = 0;
  for (x, y) in iproduct!(0..w, 0..h) {
    max_dist = max(max_dist, dist[y][x]);
  }
  println!("{}", max_dist);
}
