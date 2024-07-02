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
    q: usize,
    queries: [(usize, usize); q]
  }

  let mut a = vec![];
  for (ot, xk) in queries {
    if ot == 1 {
      a.push(xk);
    } else if ot == 2 {
      println!("{}", a[a.len() - xk]);
    }
  }
}
