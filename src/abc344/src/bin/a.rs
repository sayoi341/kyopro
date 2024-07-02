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
    s: Chars,
  }

  let mut flag = true;
  for c in s {
    if c == '|' {
      flag = !flag;
    } else if flag {
      print!("{}", c);
    }
  }
}
