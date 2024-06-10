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
    h: usize,
    x: usize,
    p: [usize; n]
  }

  for (i, pp) in p.iter().enumerate() {
    if pp + h >= x {
      println!("{}", i + 1);
      return;
    }
  }
}
