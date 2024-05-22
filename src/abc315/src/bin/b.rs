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
    m: usize,
    d: [usize; m]
  }
  let sum = d.iter().sum::<usize>();
  let mut center = (sum + sum % 2) / 2;

  for (i, dd) in d.iter().enumerate() {
    if center > *dd {
      center -= dd;
    } else {
      println!("{} {}", i + 1, center);
      return;
    }
  }

  println!("{}", center);
}
