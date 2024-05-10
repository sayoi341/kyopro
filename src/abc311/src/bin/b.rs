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
    n: usize,
    d: usize,
    s: [Chars; n]
  }

  let mut ans = 0;
  let mut cnt = 0;

  for i in 0..d {
    let mut is_ok_day = true;
    for p in &s {
      if p[i] == 'x' {
        is_ok_day = false;
        break;
      }
    }

    if is_ok_day {
      cnt += 1;
    } else {
      cnt = 0;
    }

    ans = max(ans, cnt);
  }

  println!("{}", ans);
}
