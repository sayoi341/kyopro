#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::*;
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  mem::swap,
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};

fn main() {
  input! {
    _n: usize,
    mut x: usize,
    mut y: usize,
    z: usize
  }

  if x > y {
    swap(&mut x, &mut y);
  }

  for i in x..=y {
    if i == z {
      println!("Yes");
      return;
    }
  }

  println!("No");
}
