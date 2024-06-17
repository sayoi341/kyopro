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
    m: usize,
    ss: [Chars; n],
  }

  let mut ans = 11;

  for b in 0..(1 << n) {
    let mut marus = vec![false; m];
    for (i, shop) in ss.iter().enumerate() {
      if (b >> i) & 1 == 0 {
        continue;
      }
      for (j, item) in shop.iter().enumerate() {
        if *item == 'o' {
          marus[j] = true;
        }
      }
    }
    if marus.iter().all(|&x| x) {
      ans = min(ans, (b as u32).count_ones() as i32);
    }
  }

  println!("{}", ans);
}
