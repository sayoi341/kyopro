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
    mut aarr: [isize; n]
  }

  let mut aarr_idx = aarr.iter().enumerate().map(|(i, &a)| (i + 1, a)).collect::<Vec<_>>();

  aarr_idx.sort_by_key(|&(_, a)| a);

  let mut tmp = aarr_idx[0].0;

  for _ in 0..n - 1 {
    print!("{} ", tmp);
    let tmp_idx = aarr_idx.binary_search_by_key(&tmp, |&(_, a)| a as usize).unwrap();
    tmp = aarr_idx[tmp_idx].0;
  }

  println!("{}", tmp);
}
