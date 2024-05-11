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

fn main() {
  input! {
    n: usize,
    h: [usize; n]
  }

  for (i, height) in h.iter().skip(1).enumerate() {
    if *height > h[0] {
      println!("{}", i + 2);
      return;
    }
  }

  println!("-1");
}
