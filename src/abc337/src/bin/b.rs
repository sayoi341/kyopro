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
    mut s: Chars,
  }

  let mut s = VecDeque::from_iter(s.into_iter());
  let mut flag = 0;
  let abc = ['A', 'B', 'C'];

  while let Some(c) = s.pop_front() {
    if c != abc[flag] {
      flag += 1;
      s.push_front(c);
    }

    if flag == 3 {
      break;
    }
  }

  println!("{}", if s.is_empty() { "Yes" } else { "No" });
}
