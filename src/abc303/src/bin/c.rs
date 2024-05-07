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
    _n: usize,
    m: usize,
    mut h: isize, // 体力
    k: usize,
    s: Chars,
    xy: [(isize, isize); m],
  }

  let mut set = BTreeSet::from_iter(xy);
  let mut takahashi = (0isize, 0isize);

  for c in s {
    match c {
      'R' => takahashi.0 += 1,
      'L' => takahashi.0 -= 1,
      'U' => takahashi.1 += 1,
      'D' => takahashi.1 -= 1,
      _ => unreachable!(),
    }

    h -= 1;

    if h < 0 {
      println!("No");
      return;
    }

    if set.contains(&takahashi) && h < k as isize {
      h = k as isize;
      set.remove(&takahashi);
    }
  }

  println!("Yes");
}
