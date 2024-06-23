#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use smallvec::Array;
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
    aarr: [usize; 2*n]
  }

  let mut ans = 0;

  for i in 0..2 * n - 2 {
    if aarr[i] == aarr[i + 2] && aarr[i] != aarr[i + 1] {
      ans += 1;
    }
  }

  println!("{}", ans);
}
