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
    a: [usize; 9],
    b:[usize; 8],
  }

  let a_sum = a.iter().sum::<usize>();
  let b_sum = b.iter().sum::<usize>();

  println!("{}", a_sum - b_sum + 1);
}
