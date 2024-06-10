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
    mut a: [usize; n]
  }
  a.sort();

  let min = a[0];

  for (i, ari) in (min..(min + n + 1)).enumerate() {
    if ari != a[i] {
      println!("{}", ari);
      return;
    }
  }
}
