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
    xy: [(usize, usize); n]
  }
  let xsum = xy.iter().map(|(x, _)| x).sum::<usize>();
  let ysum = xy.iter().map(|(_, y)| y).sum::<usize>();

  println!(
    "{}",
    if xsum == ysum {
      "Draw"
    } else if xsum > ysum {
      "Takahashi"
    } else {
      "Aoki"
    }
  );
}
