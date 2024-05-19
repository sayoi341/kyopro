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
    p: [usize; n]
  }

  let max = p.iter().max().unwrap();
  let maxi = p.iter().position(|&x| x == *max).unwrap();
  let maxc = p.iter().filter(|&&x| x == *max).count();

  if maxi == 0 && maxc == 1 {
    println!("0");
  } else {
    println!("{}", max - p[0] + 1);
  }
}
