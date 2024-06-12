#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use rand_core::le;
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
    m: usize,
    mut l: [usize; n],
  }

  l = l.iter().map(|x| x + 1).collect::<Vec<_>>();

  // 真である条件：rows <= m

  let mut lower_width = l.iter().max().unwrap() - 1; // 偽になる行の幅
  let mut upper_width = l.iter().sum::<usize>(); // 真になる行の幅

  while lower_width + 1 < upper_width {
    let mid_width = (lower_width + upper_width) / 2;

    let mut rows = 1;
    let mut length = 0;

    for word in &l {
      length += word;
      if length > mid_width {
        rows += 1;
        length = *word;
      }
    }

    if rows <= m {
      upper_width = mid_width; // mid_widthも真である
    } else {
      lower_width = mid_width; // mid_widthも偽である
    }
  }

  println!("{}", upper_width - 1);
}
