#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::*;
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
    a: usize,
    b: usize,
    c_arr: [usize; n],
  }
  let mut ans = 0;

  for (i, c) in c_arr.iter().enumerate() {
    if *c == a + b {
      ans = i;
    }
  }

  println!("{}", ans + 1);
}
