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
    mut m: isize,
    ha: [isize; n],
  }

  for (i, h) in ha.iter().enumerate() {
    m -= h;
    if m < 0 {
      println!("{i}");
      return;
    }
  }

  println!("{n}");
}
