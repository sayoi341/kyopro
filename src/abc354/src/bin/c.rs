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
    mut ac: [(usize, usize); n]
  }

  let mut i_ac = ac.iter().enumerate().map(|(i, &(a, c))| (i, a, c)).collect::<Vec<_>>();

  // cを昇順にソート
  i_ac.sort_by(|a, b| a.2.cmp(&b.2));
  let mut ans_set: BTreeSet<usize> = BTreeSet::new();
  let mut max_a = 0;

  for e in i_ac {
    if max_a < e.1 {
      ans_set.insert(e.0);
    }

    max_a = max(max_a, e.1);
  }

  println!("{}", ans_set.len());
  println!("{}", ans_set.iter().map(|&x| x + 1).join(" "));
}
