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
    sc: [(Chars, usize); n],
  }

  let sum = sc.iter().map(|(_, c)| c).sum::<usize>();
  let mut s_arr: Vec<String> = sc.iter().map(|(s, _)| s).map(|s| s.iter().collect()).collect();
  s_arr.sort();

  let index = sum % n;

  println!("{}", s_arr[index]);
}
