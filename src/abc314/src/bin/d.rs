#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use libm::fabs;
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
    _n: usize,
    mut s: Chars,
    q: usize,
    txc: [(usize, usize, char); q],
  }

  let mut last23 = 0;
  for (i, (t, _, _)) in txc.iter().enumerate() {
    if *t == 2 || *t == 3 {
      last23 = max(i, last23);
    }
  }

  for (i, (t, x, c)) in txc.iter().enumerate() {
    if *t == 1 {
      s[*x - 1] = *c;
    }
    if i == last23 {
      if *t == 2 {
        // sを全て小文字にする
        for cc in &mut s {
          *cc = cc.to_ascii_lowercase();
        }
      } else if *t == 3 {
        // sを全て大文字にする
        for cc in &mut s {
          *cc = cc.to_ascii_uppercase();
        }
      }
    }
  }

  println!("{}", s.iter().collect::<String>());
}
