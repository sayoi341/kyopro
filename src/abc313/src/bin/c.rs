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
use superslice::Ext;

fn main() {
  input! {
    n: usize,
    mut a: [usize; n]
  }

  a.sort();
  // sumが不変条件
  let sum = a.iter().sum::<usize>();
  // lower, upperの値もsumから決まる
  let lower = sum / n;
  let upper = sum / n + 1;

  // lower, upperの個数も決まる
  let upper_count = sum % n;
  let lower_count = n - upper_count;

  // lowerがlower_count個, upperがupper_count個
  let mut ans_vec = vec![lower; lower_count];
  ans_vec.append(&mut vec![upper; upper_count]);

  let mut ans = 0;

  for (from, to) in a.iter().zip(ans_vec.iter()) {
    ans += from.abs_diff(*to);
  }

  println!("{}", ans / 2);
}
