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
    abcd: [(usize, usize, usize, usize); n],
  }

  let mut s = [[false; 100]; 100];

  for (a, b, c, d) in abcd {
    for l in s.iter_mut().skip(a).take(b - a) {
      for c in l.iter_mut().skip(c).take(d - c) {
        *c = true;
      }
    }
  }

  let mut ans = 0;

  for l in &s {
    for c in l {
      if *c {
        ans += 1;
      }
    }
  }

  println!("{}", ans);
}
