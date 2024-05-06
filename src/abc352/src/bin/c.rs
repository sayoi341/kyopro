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
    ab: [(usize, usize); n],
  }

  let mut a_sum = 0;
  for (a, _) in ab.clone() {
    a_sum += a;
  }

  // b - a の最大値を求める
  let mut b_max = 0;
  for (a, b) in ab {
    b_max = max(b_max, b - a);
  }

  println!("{}", b_max + a_sum);
}
