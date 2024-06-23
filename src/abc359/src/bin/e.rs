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
    hs: [usize; n]
  }

  let mut vec = Vec::new();
  vec.push(hs[0]);

  for i in 1..n {
    // hで累積和を取る
    vec.push(vec[i - 1] + hs[i]);

    // hs[i]以上の値を持つインデックスを、iより前で二分探索
    let idx = vec[..i].lower_bound(&hs[i]);
    println!("idx: {}", idx);
    if idx < i {
      vec[i] = hs[i] * (i - idx + 1);
    }
  }

  println!("{}", vec.iter().map(|&x| x + 1).join(" "));
}
