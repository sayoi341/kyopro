#![allow(unused_imports)]
use ac_library::*;
use itertools::{iproduct, Itertools, MinMaxResult};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [[Usize1; n]; m],
  }

  let mut set = BTreeSet::new();
  for (i, j) in iproduct!(0..n, 0..n) {
    if i == j {
      continue;
    }
    set.insert((min(i, j), max(i, j)));
  }

  for line in a {
    for i in 0..n - 1 {
      set.remove(&(min(line[i], line[i + 1]), max(line[i], line[i + 1])));
    }
  }

  println!("{}", set.len());
}
