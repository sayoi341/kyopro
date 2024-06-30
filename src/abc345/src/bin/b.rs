#![allow(unused_imports)]
#![allow(unstable_name_collisions)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use num::Integer;
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
    mut x: isize,
  }

  println!("{}", (x).div_ceil(&10));
}
