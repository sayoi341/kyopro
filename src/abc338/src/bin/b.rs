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

  let mut alphabet = vec![0; 26];
  for c in s {
    alphabet[c as usize - 'a' as usize] += 1;
  }

  let mut max_idx = 0;
  for (i, c) in alphabet.iter().enumerate().rev() {
    if alphabet[max_idx] <= *c {
      max_idx = i;
    }
  }

  println!("{}", (max_idx as u8 + 'a' as u8) as char);
}
