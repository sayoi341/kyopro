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
    s: Chars,
  }

  let n = s.len();

  let mut alphabets = [0usize; 26];
  for c in s {
    alphabets[(c as u8 - b'a') as usize] += 1;
  }

  let mut ans = n * n;
  let mut nochange = false;

  for ab in alphabets.iter() {
    if *ab > 1 {
      nochange = true;
    }
    ans -= ab * ab;
  }

  ans /= 2;

  if nochange {
    ans += 1;
  }

  println!("{}", ans)
}
