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

  if s[0] == '<' && s[s.len() - 1] == '>' && s[1..s.len() - 1].iter().all(|&c| c == '=') {
    println!("Yes");
  } else {
    println!("No");
  }
}
