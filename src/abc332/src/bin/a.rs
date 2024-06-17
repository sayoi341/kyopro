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
    s: usize,
    k: usize,
    pqs: [(usize, usize); n],
  }

  let sum = pqs.iter().map(|(p, q)| p * q).sum::<usize>();

  println!("{}", if sum >= s { sum } else { sum + k });
}
