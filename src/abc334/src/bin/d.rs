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
    q: usize,
    mut rarr: [usize; n],
    qarr: [usize; q],
  }

  rarr.sort();
  // rarrで累積和を取る
  let mut rsum = Vec::new();
  rsum.push(rarr[0]);
  for i in 1..n {
    rsum.push(rsum[i - 1] + rarr[i]);
  }

  //println!("{:?}", rsum);

  for q in qarr {
    let ans = rsum.upper_bound(&q);
    println!("{}", ans);
  }
}
