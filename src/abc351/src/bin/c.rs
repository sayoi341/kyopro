#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::*;
use rand_core::le;
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
    a_tmp: [usize; n],
  }

  let mut right: VecDeque<usize> = VecDeque::from(a_tmp);
  let mut left: Vec<usize> = Vec::new();

  while let Some(a) = right.pop_front() {
    if left.is_empty() {
      left.push(a);
      continue;
    }

    if *left.last().unwrap() == a {
      left.pop();
      right.push_front(a + 1);
    } else {
      left.push(a);
    }
  }

  println!("{}", left.len());
}
