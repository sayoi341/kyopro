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
    _l: usize,
    n1: usize,
    n2: usize,
    a: [(usize, isize); n1],
    b: [(usize, isize); n2],
  }

  let mut index_a = 0;
  let mut index_b = 0;
  let mut count_a = a[index_a].1;
  let mut count_b = b[index_b].1;

  let mut ans = 0;

  while index_a < n1 || index_b < n2 {
    let val_a = if index_a < n1 { a[index_a].0 } else { 0 };
    let val_b = if index_b < n2 { b[index_b].0 } else { 0 };

    if count_a < count_b {
      if val_a == val_b {
        ans += min(count_a, count_b);
      }
      count_b -= count_a;
      count_a = 0;
    } else {
      if val_a == val_b {
        ans += min(count_a, count_b);
      }
      count_a -= count_b;
      count_b = 0;
    }

    if count_a <= 0 {
      index_a += 1;
      count_a = if index_a < n1 { a[index_a].1 } else { 0 };
    }

    if count_b <= 0 {
      index_b += 1;
      count_b = if index_b < n2 { b[index_b].1 } else { 0 };
    }
  }

  println!("{}", ans);
}
