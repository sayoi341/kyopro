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
    m: usize,
    ss: Chars,
    ts: Chars,
  }

  let mut flag_f = false;
  let mut flag_b = false;

  if ss == ts.iter().take(n).map(|&c| c).collect::<Vec<_>>() {
    flag_f = true;
  }

  if ss == ts.iter().skip(m - n).map(|&c| c).collect::<Vec<_>>() {
    flag_b = true;
  }

  if flag_f && flag_b {
    println!("0");
  } else if flag_f {
    println!("1");
  } else if flag_b {
    println!("2");
  } else {
    println!("3");
  }
}
