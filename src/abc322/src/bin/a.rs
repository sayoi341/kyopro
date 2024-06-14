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
    cs: Chars
  }

  for i in 0..n - 2 {
    if cs[i] == 'A' && cs[i + 1] == 'B' && cs[i + 2] == 'C' {
      println!("{}", i + 1);
      return;
    }
  }

  println!("-1");
}
