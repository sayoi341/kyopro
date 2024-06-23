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
    ss: [Chars; n]
  }

  let mut ans = 0;
  for s in ss {
    if s == "Takahashi".chars().collect::<Vec<_>>() {
      ans += 1;
    }
  }

  println!("{}", ans);
}
