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
    qarr: [isize; n],
    mut aarr: [isize; n],
    mut barr: [isize; n],
  }

  let mut ans = 0;

  let qmax = qarr.iter().max().unwrap();

  for x in 0..=*qmax {
    let mut y = isize::MAX;
    for i in 0..n {
      if qarr[i] < aarr[i] * x {
        y = -isize::MAX;
      } else if barr[i] > 0 {
        y = min(y, (qarr[i] - aarr[i] * x) / barr[i]);
      }
    }
    ans = max(ans, x + y);
  }

  println!("{}", ans);
}
