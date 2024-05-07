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
    mut p: char,
    mut q: char,
  }

  (p, q) = (min(p, q), max(p, q));

  let mut ans = 0;
  let mut flag = false;

  for (c, v) in [('A', 3), ('B', 1), ('C', 4), ('D', 1), ('E', 5), ('F', 9), ('G', 0)] {
    if c == q {
      break;
    }
    if c == p {
      flag = true;
    }
    if flag {
      ans += v;
    }
  }

  println!("{}", ans);
}
