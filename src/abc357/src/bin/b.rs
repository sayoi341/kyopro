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
    cs: Chars,
  }

  let mut upper = 0;
  let mut lower = 0;

  for c in &cs {
    if c.is_uppercase() {
      upper += 1;
    } else {
      lower += 1;
    }
  }

  if upper > lower {
    println!("{}", cs.iter().map(|&c| c.to_ascii_uppercase()).collect::<String>());
  } else {
    println!("{}", cs.iter().map(|&c| c.to_ascii_lowercase()).collect::<String>());
  }
}
