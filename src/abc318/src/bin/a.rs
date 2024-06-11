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
    m: usize,
    p: usize,
  }

  let mut day = n;

  if n < m {
    println!("0");
    return;
  }

  day -= m;
  let mut ans = 1;
  ans += day / p;

  println!("{}", ans);
}
