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

fn floor(x: isize, m: isize) -> isize {
  let r = (x % m + m) % m;
  (x - r) / m
}

fn main() {
  input! {
    a: isize, // 基点
    m: isize, // aから間隔mずつで配置
    mut l: isize, // 始点
    mut r: isize, // 終点
  }

  l -= a;
  r -= a;

  println!("{}", floor(r, m) - floor(l - 1, m));
}
