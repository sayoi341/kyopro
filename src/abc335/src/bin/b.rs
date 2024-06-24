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
  }

  let mut x = 0;
  let mut y = 0;
  let mut z = 0;

  while z != n {
    println!("{} {} {}", z, y, x);
    x += 1;
    if x + y + z > n {
      x = 0;
      y += 1;
      if x + y + z > n {
        x = 0;
        y = 0;
        z += 1;
      }
    }
  }
  println!("{} {} {}", z, y, x);
}
