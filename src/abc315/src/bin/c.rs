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
    mut f_s: [(usize, usize); n]
  }

  let max = f_s.iter().fold((0, 0), |acc, e| if e.1 > acc.1 { *e } else { acc });
  if let Some(remove_index) = f_s.iter().position(|x| x.0 == max.0 && x.1 == max.1) {
    f_s.remove(remove_index);
  }

  let same_max = f_s.iter().filter(|x| x.0 == max.0).fold((0, 0), |acc, e| if e.1 > acc.1 { *e } else { acc });
  let not_same_max = f_s.iter().filter(|x| x.0 != max.0).fold((0, 0), |acc, e| if e.1 > acc.1 { *e } else { acc });

  if same_max.1 / 2 > not_same_max.1 {
    println!("{}", max.1 + same_max.1 / 2);
  } else {
    println!("{}", max.1 + not_same_max.1);
  }
}
