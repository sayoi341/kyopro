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

fn main() {
  input! {
    n: usize,
    a:[Chars; n],
    b:[Chars; n],
  }

  for i in 0..n {
    for j in 0..n {
      if a[i][j] != b[i][j] {
        println!("{} {}", i + 1, j + 1);
      }
    }
  }
}
