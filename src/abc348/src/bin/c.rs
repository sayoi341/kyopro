#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use num_traits::Pow;
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
    mut acarr: [(usize, usize); n]
  }

  acarr.sort();

  let mut map: BTreeMap<usize, usize> = BTreeMap::new();

  for (a, c) in acarr {
    // cを検索し、なければaを追加、あればmin(a, map[c])
    let min_a = map.get(&c).map_or(a, |&x| min(x, a));
    map.insert(c, min_a);
  }

  // valueの最大値を取得
  println!("{}", map.values().max().unwrap());
}
