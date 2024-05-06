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

fn main() {
  input! {
    s: Chars,
    t: Chars,
  }

  let mut s_count = 0;

  for (i, tc) in t.iter().enumerate() {
    if s[s_count] == *tc {
      print!("{} ", i + 1);
      s_count += 1;
    }
  }
}
