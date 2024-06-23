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
    m: usize,
    aarr: [usize; m],
    x: [[usize; m]; n],
  }

  let mut vec = vec![0usize; m];

  for xx in x {
    for (i, &x) in xx.iter().enumerate() {
      vec[i] += x;
    }
  }

  for (i, v) in vec.iter().enumerate() {
    if *v < aarr[i] {
      println!("No");
      return;
    }
  }

  println!("Yes");
}
