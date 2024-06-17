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
    a: usize,
    ts: [usize; n],
  }

  let mut timer = 0;

  for t in ts {
    if timer >= t {
      timer += a;
      println!("{}", timer);
    } else {
      timer = t + a;
      println!("{}", timer);
    }
  }
}
