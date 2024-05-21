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
    n: usize,
  }

  let s: Vec<char> = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679"
    .chars()
    .collect();

  println!("3.{}", s.iter().take(n).collect::<String>());
}
