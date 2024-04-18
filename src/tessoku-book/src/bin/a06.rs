#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use proconio::{input, marker::*};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
  vec,
};

fn main() {
  input! {
      n: usize,
      q: usize,
      aa: [usize; n],
      lr: [(usize, usize); q],
  }

  let mut acc = vec![0; n + 1];
  for i in 0..n {
    acc[i + 1] = acc[i] + aa[i];
  }

  for (l, r) in lr {
    println!("{}", acc[r] - acc[l - 1]);
  }
}
