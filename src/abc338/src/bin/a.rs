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
    s: Chars
  }

  println!(
    "{}",
    if s[0].is_ascii_uppercase() && s[1..].iter().all(|&c| c.is_ascii_lowercase()) {
      "Yes"
    } else {
      "No"
    }
  );
}
