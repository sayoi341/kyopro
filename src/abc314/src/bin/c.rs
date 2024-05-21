#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  mem,
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
  vec,
};
use superslice::Ext;

fn main() {
  input! {
    n: usize,
    m: usize,
    mut s: Chars,
    c: [Usize1; n],
  }

  let mut before_color_char = vec![' '; m];
  let mut is_changed = vec![false; n];

  for i in 0..n {
    if is_changed[i] {
      continue;
    }

    if before_color_char[c[i]] == ' ' {
      before_color_char[c[i]] = s[i];
      continue;
    }

    before_color_char[c[i]] = mem::replace(&mut s[i], before_color_char[c[i]]);
    is_changed[i] = true;
  }

  for i in 0..n {
    if is_changed[i] {
      continue;
    }

    if before_color_char[c[i]] == ' ' {
      before_color_char[c[i]] = s[i];
      continue;
    }

    before_color_char[c[i]] = mem::replace(&mut s[i], before_color_char[c[i]]);
    is_changed[i] = true;
  }

  println!("{}", s.iter().collect::<String>());
}
