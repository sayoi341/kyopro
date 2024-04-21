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

#[fastout]
fn main() {
  input! {
    n: usize,
    mut a_arr: [Usize1; n],
  }

  let mut pos = vec![0; n];
  for (i, &a) in a_arr.iter().enumerate() {
    pos[a] = i;
  }

  let mut swap_log_set: BTreeSet<(usize, usize)> = BTreeSet::new();

  for i in 0..n {
    if pos[i] == i {
      continue;
    }

    let j = pos[i];
    let k = a_arr[i];

    swap_log_set.insert((i, j));
    a_arr.swap(i, j);
    pos.swap(i, k);
  }

  println!("{}", swap_log_set.len());
  for (i, j) in swap_log_set {
    println!("{} {}", i + 1, j + 1);
  }
}
