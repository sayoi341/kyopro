#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};

fn main() {
  input! {
    n: usize,
    k: usize,
    p: [usize; n],
  }

  let mut q = vec![0; n];
  for (i, pe) in p.iter().enumerate() {
    q[*pe - 1] = i;
  }

  let mut set = BTreeSet::new();
  for qe in q.iter().take(k) {
    set.insert(*qe);
  }

  // 最大値 : set.iter().max().unwrap()
  // 最小値 : set.iter().min().unwrap()
  let mut ans = set.iter().max().unwrap() - set.iter().min().unwrap();

  for i in k..n {
    set.remove(&q[i - k]);
    set.insert(q[i]);
    ans = min(ans, set.iter().max().unwrap() - set.iter().min().unwrap());
  }

  println!("{}", ans);
}
