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
    k: usize,
    mut aarr: [usize; n],
  }

  aarr.sort();
  aarr.dedup();

  // 1からkの和を求める
  let mut sum = k * (k + 1) / 2;

  for a in aarr {
    if a > k {
      break;
    }

    sum -= a;
  }

  println!("{}", sum);
}
