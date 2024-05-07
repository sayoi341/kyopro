#![allow(unused_imports)]
use ac_library::*;
use itertools::{concat, Itertools, MinMaxResult};
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
    s: Chars,
    t: Chars,
  }

  let mut ans = true;

  for i in 0..n {
    if s[i] != t[i] {
      if s[i] == 'o' && t[i] == '0' {
        continue;
      }

      if s[i] == '0' && t[i] == 'o' {
        continue;
      }

      if s[i] == 'l' && t[i] == '1' {
        continue;
      }

      if s[i] == '1' && t[i] == 'l' {
        continue;
      }

      ans = false;
      break;
    }
  }

  println!("{}", if ans { "Yes" } else { "No" });
}
