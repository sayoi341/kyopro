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

fn main() {
  input! {
    n: usize,
    mut a: [usize; n]
  }
  // 10^8 = 100'000'000

  a.sort();

  let mut ans = a.iter().sum::<usize>() * (n - 1);

  let mut count = 0;
  for (i, aa) in a.iter().enumerate() {
    let nokori = 100000000 - aa;
    let b = a
      .binary_search_by(|x: &usize| x.cmp(&nokori).then(std::cmp::Ordering::Greater))
      .unwrap_or_else(|x| x);
    count += min(n - b, n - i - 1);
  }

  ans -= 100000000 * count;

  println!("{}", ans);
}
