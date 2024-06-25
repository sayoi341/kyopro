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
    n: usize
  }

  let binary = format!("{:b}", n);

  let mut ans = 0;

  for c in binary.chars().rev() {
    if c == '0' {
      ans += 1;
    } else {
      break;
    }
  }

  println!("{ans}");
}
