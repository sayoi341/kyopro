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
    a: [usize; n],
    b: [usize; m],
  }

  let mut count_a = 0usize;
  let mut count_b = 0usize;

  let mut vec_a = vec![0; n];
  let mut vec_b = vec![0; m];

  for i in 0..n + m {
    if count_a == n {
      vec_b[count_b] = i + 1;
      count_b += 1;
      continue;
    }

    if count_b == m {
      vec_a[count_a] = i + 1;
      count_a += 1;
      continue;
    }

    if a[count_a] < b[count_b] {
      vec_a[count_a] = i + 1;
      count_a += 1;
    } else {
      vec_b[count_b] = i + 1;
      count_b += 1;
    }
  }

  println!("{}", vec_a.iter().join(" "));
  println!("{}", vec_b.iter().join(" "));
}
