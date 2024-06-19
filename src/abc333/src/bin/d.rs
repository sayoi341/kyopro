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
use superslice::Ext;

fn main() {
  input! {
    n: usize,
    uv: [(Usize1, Usize1); n - 1],
  }

  let mut dsu = Dsu::new(n);
  for &(u, v) in &uv {
    if u == 0 {
      continue;
    }
    dsu.merge(u, v);
  }

  let mut ans = vec![0; n];
  for i in 0..n {
    ans[dsu.leader(i)] += 1;
  }
  ans.remove(0);

  let mut anss = usize::MAX;

  for c in ans {
    if c == 0 {
      continue;
    }
    anss = min(anss, n - c);
  }

  println!("{}", anss);
}
