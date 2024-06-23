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
    mut sx: usize,
    sy: usize,
    mut tx: usize,
    ty: usize
  }
  if (sx + sy) % 2 == 1 {
    sx -= 1;
  }
  if (tx + ty) % 2 == 1 {
    tx -= 1;
  }

  let dx = tx.abs_diff(sx);
  let dy = ty.abs_diff(sy);

  println!("{}", (dy + max(dx, dy)) / 2);
}
