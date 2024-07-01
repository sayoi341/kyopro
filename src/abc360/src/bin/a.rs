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
    s: Chars,
  }

  let mut ridx = 0;
  let mut midx = 0;

  for (i, c) in s.iter().enumerate() {
    if *c == 'R' {
      ridx = i;
    } else if *c == 'M' {
      midx = i;
    }
  }

  println!("{}", if ridx < midx { "Yes" } else { "No" });
}
