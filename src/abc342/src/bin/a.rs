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

  let c1 = s[0];
  let mut idx1 = 1isize;
  let mut c2 = ' ';
  let mut idx2 = -1;

  for (idx, c) in s[1..].iter().enumerate() {
    if c1 == *c {
      idx1 = -1;
    } else {
      if c2 == *c {
        idx2 = -1;
      } else {
        c2 = *c;
        idx2 = idx as isize + 2;
      }
    }
  }

  // -1じゃない方を出力
  if idx1 != -1 {
    println!("{}", idx1);
  } else {
    println!("{}", idx2);
  }
}
