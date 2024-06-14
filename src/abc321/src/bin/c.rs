#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use rand::rngs::adapter;
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
    k: usize,
  }

  let mut v: Vec<usize> = Vec::new();

  for bit in 2..(1 << 10) {
    let mut num = 0;
    for i in (0..10).rev() {
      if bit & (1 << i) != 0 {
        num = num * 10 + i;
      }
    }
    v.push(num);
  }

  v.sort();

  println!("{}", v[k - 1]);
}
