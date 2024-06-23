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
    mut lrarr: [(usize, usize);n],
  }

  lrarr.sort();

  let mut left = lrarr.iter().map(|(l, _)| *l).collect::<Vec<_>>();
  left.sort();

  let mut ans = 0;

  for (i, (_, r)) in lrarr.iter().enumerate() {
    let j = left.upper_bound(&r) - 1;
    ans += j - i;
  }

  println!("{}", ans);
}
