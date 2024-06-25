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
    n: usize
  }

  // nを5進数に変換
  let mut n = n - 1;
  let mut ans = VecDeque::new();
  while n >= 5 {
    ans.push_front(n % 5);
    n /= 5;
  }
  ans.push_front(n);

  println!("{}", ans.iter().map(|&x| x * 2).join(""));
}
