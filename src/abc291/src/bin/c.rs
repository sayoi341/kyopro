#![allow(unused_imports)]
use ac_library::Dsu;
use itertools::{Itertools, MinMaxResult, Positions};
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
      _n: usize,
      s: Chars
  }

  let mut position = (0isize, 0isize);
  let mut position_set: HashSet<(isize, isize)> = HashSet::new();

  position_set.insert(position);

  for c in s {
    match c {
      'L' => position.0 += 1,
      'R' => position.0 -= 1,
      'U' => position.1 += 1,
      'D' => position.1 -= 1,
      _ => {}
    }

    if position_set.contains(&position) {
      println!("Yes");
      return;
    }

    position_set.insert(position);
  }

  println!("No");
}
