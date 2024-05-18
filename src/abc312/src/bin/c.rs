#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use num_traits::WrappingNeg;
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
    m: usize,
    mut a: [usize; n],
    mut b: [usize; m],
  }

  a.sort();
  b.sort();

  let mut left = 0;
  let mut right = 10e9 as usize;

  while right - left > 1 {
    let mid = (left + right) / 2;

    let sell = a.upper_bound(&mid);
    let buy = m - b.lower_bound(&mid);

    if sell >= buy {
      right = mid;
    } else {
      left = mid;
    }
  }

  println!("{}", right);
}
