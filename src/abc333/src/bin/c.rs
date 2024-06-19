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
  vec,
};
use superslice::Ext;

fn main() {
  input! {
    n: usize
  }

  let mut pyu: Vec<usize> = Vec::with_capacity(13);
  pyu.push(1);

  for i in 0..12 {
    pyu.push(pyu[i] * 10 + 1);
  }

  let mut ansv: Vec<usize> = Vec::new();

  for comb in (0..13).combinations_with_replacement(3) {
    //println!("{:?}", comb);
    ansv.push(pyu[comb[0]] + pyu[comb[1]] + pyu[comb[2]]);
  }

  ansv.sort();

  println!("{}", ansv[n - 1]);
}
