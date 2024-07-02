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
    mut aarr: [usize; n],
    st: [(usize, usize); n-1]
  }

  for i in 0..n - 1 {
    if aarr[i] >= st[i].0 {
      aarr[i + 1] += st[i].1 * (aarr[i] / st[i].0);
      aarr[i] %= st[i].0;
    }
  }

  println!("{}", aarr[n - 1]);
}
