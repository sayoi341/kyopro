#![allow(unused_imports)]
#![allow(unstable_name_collisions)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use memoise::memoise_map;
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

#[memoise_map(n)]
fn f(n: usize) -> usize {
  if n == 1 {
    return 0;
  }
  f(n / 2) + f(n / 2 + n % 2) + n
}

fn main() {
  input! {
    n: usize,
  }

  println!("{}", f(n));
}
