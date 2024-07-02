#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use num_integer::Roots;
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
  }

  // 1 <= i <= 3√n
  for i in (1..=n.nth_root(3)).rev() {
    // i * i * iがiが回文立法数であるかを判定
    let x = i * i * i;
    let cs = x.to_string().chars().collect::<Vec<_>>();
    let mut ok = true;
    for j in 0..cs.len() / 2 {
      if cs[j] != cs[cs.len() - j - 1] {
        ok = false;
        break;
      }
    }
    if ok {
      println!("{}", x);
      return;
    }
  }
}
