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
  }

  let mut s = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
  let last = s.len() - 1;

  if s[last] <= 2 {
    s[last] = 0;
  } else {
    s[last] = 5;
  }

  println!("{}", s.iter().join(""));
}
