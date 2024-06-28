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
  vec,
};
use superslice::Ext;

fn main() {
  input! {
    n: usize,
    xy: [(isize, isize); n]
  }

  for i in 0..n {
    let mut maxidx = 0;
    let mut maxdist = 0;
    let (x1, y1) = xy[i];
    for (i, (x2, y2)) in xy.iter().enumerate() {
      let dist = ((x1 - x2).pow(2) + (y1 - y2).pow(2)).abs();
      if dist > maxdist {
        maxdist = dist;
        maxidx = i;
      }
    }

    println!("{}", maxidx + 1);
  }
}
