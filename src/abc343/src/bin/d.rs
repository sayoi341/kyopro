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
    t: usize,
    abarr: [(Usize1, usize); t],
  }

  let mut map = BTreeMap::new();
  map.entry(0).or_insert(n);
  let mut scores = vec![0; n];

  for (player, add_score) in abarr {
    let current_score = scores[player];
    map.entry(current_score).and_modify(|v| *v -= 1);
    if map[&current_score] == 0 {
      map.remove(&current_score);
    }
    scores[player] += add_score;
    map.entry(scores[player]).and_modify(|v| *v += 1).or_insert(1);
    println!("{}", map.len());
  }
}
